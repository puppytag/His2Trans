#!/usr/bin/env python3
"""
GPU 任务队列调度器

实现功能：
1. 动态监控每个 GPU 的内存使用情况
2. 将任务分配给最空闲的 GPU
3. 内存不足时自动等待
4. 支持优先级队列
5. 优雅处理 OOM

使用方法：
    # 作为独立服务运行
    python gpu_task_scheduler.py --mode server
    
    # 提交任务
    python gpu_task_scheduler.py --mode submit --project my_project
    
    # 查看 GPU 状态
    python gpu_task_scheduler.py --mode status
"""

import os
import sys
import time
import json
import queue
import threading
import subprocess
from pathlib import Path
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple
from datetime import datetime

# 尝试导入 torch（用于 GPU 监控）
try:
    import torch
    TORCH_AVAILABLE = True
except ImportError:
    TORCH_AVAILABLE = False
    print("警告: PyTorch 不可用，GPU 监控功能受限")


@dataclass
class GPUStatus:
    """GPU 状态信息"""
    gpu_id: int
    total_memory_gb: float = 0.0
    used_memory_gb: float = 0.0
    free_memory_gb: float = 0.0
    utilization_percent: float = 0.0
    current_task: Optional[str] = None
    last_updated: float = field(default_factory=time.time)
    
    @property
    def is_busy(self) -> bool:
        return self.current_task is not None
    
    @property
    def memory_available_for_task(self) -> bool:
        """检查是否有足够内存运行 Jina Reranker（约需 8GB）"""
        MIN_MEMORY_GB = 8.0
        return self.free_memory_gb >= MIN_MEMORY_GB


@dataclass
class Task:
    """任务定义"""
    task_id: str
    project_name: str
    priority: int = 0  # 数字越大优先级越高
    created_at: float = field(default_factory=time.time)
    started_at: Optional[float] = None
    completed_at: Optional[float] = None
    assigned_gpu: Optional[int] = None
    status: str = "pending"  # pending, running, completed, failed
    error_message: Optional[str] = None
    
    def __lt__(self, other):
        # 优先级高的先执行，相同优先级按创建时间排序
        if self.priority != other.priority:
            return self.priority > other.priority
        return self.created_at < other.created_at


class GPUMonitor:
    """GPU 监控器"""
    
    def __init__(self):
        self.num_gpus = self._detect_gpus()
        self.gpu_status: Dict[int, GPUStatus] = {}
        self._lock = threading.Lock()
        self._init_gpu_status()
    
    def _detect_gpus(self) -> int:
        """检测可用 GPU 数量"""
        if not TORCH_AVAILABLE:
            return 0
        try:
            return torch.cuda.device_count()
        except Exception:
            return 0
    
    def _init_gpu_status(self):
        """初始化 GPU 状态"""
        for gpu_id in range(self.num_gpus):
            self.gpu_status[gpu_id] = GPUStatus(gpu_id=gpu_id)
        self.refresh_all()
    
    def refresh_gpu(self, gpu_id: int) -> GPUStatus:
        """刷新单个 GPU 状态"""
        if not TORCH_AVAILABLE or gpu_id >= self.num_gpus:
            return self.gpu_status.get(gpu_id, GPUStatus(gpu_id=gpu_id))
        
        try:
            props = torch.cuda.get_device_properties(gpu_id)
            total_memory = props.total_memory / (1024**3)  # GB
            
            # 获取已使用内存
            torch.cuda.set_device(gpu_id)
            allocated = torch.cuda.memory_allocated(gpu_id) / (1024**3)
            reserved = torch.cuda.memory_reserved(gpu_id) / (1024**3)
            
            # 使用 nvidia-smi 获取更准确的内存信息
            try:
                result = subprocess.run(
                    ['nvidia-smi', '--query-gpu=memory.used,memory.free,utilization.gpu',
                     '--format=csv,noheader,nounits', f'--id={gpu_id}'],
                    capture_output=True, text=True, timeout=5
                )
                if result.returncode == 0:
                    parts = result.stdout.strip().split(',')
                    used_mb = float(parts[0].strip())
                    free_mb = float(parts[1].strip())
                    util = float(parts[2].strip())
                    
                    used_memory = used_mb / 1024
                    free_memory = free_mb / 1024
                    utilization = util
                else:
                    used_memory = max(allocated, reserved)
                    free_memory = total_memory - used_memory
                    utilization = 0.0
            except Exception:
                used_memory = max(allocated, reserved)
                free_memory = total_memory - used_memory
                utilization = 0.0
            
            with self._lock:
                status = self.gpu_status[gpu_id]
                status.total_memory_gb = total_memory
                status.used_memory_gb = used_memory
                status.free_memory_gb = free_memory
                status.utilization_percent = utilization
                status.last_updated = time.time()
            
            return self.gpu_status[gpu_id]
            
        except Exception as e:
            print(f"刷新 GPU {gpu_id} 状态失败: {e}")
            return self.gpu_status.get(gpu_id, GPUStatus(gpu_id=gpu_id))
    
    def refresh_all(self):
        """刷新所有 GPU 状态"""
        for gpu_id in range(self.num_gpus):
            self.refresh_gpu(gpu_id)
    
    def get_best_gpu(self, min_memory_gb: float = 8.0) -> Optional[int]:
        """
        获取最适合执行任务的 GPU
        
        优先选择：
        1. 当前没有任务在运行的 GPU
        2. 空闲内存最多的 GPU
        3. 利用率最低的 GPU
        
        Args:
            min_memory_gb: 最小所需内存（GB）
            
        Returns:
            GPU ID，如果没有合适的 GPU 则返回 None
        """
        self.refresh_all()
        
        candidates = []
        for gpu_id, status in self.gpu_status.items():
            if not status.is_busy and status.free_memory_gb >= min_memory_gb:
                candidates.append((
                    gpu_id,
                    status.free_memory_gb,
                    status.utilization_percent
                ))
        
        if not candidates:
            return None
        
        # 按空闲内存降序、利用率升序排序
        candidates.sort(key=lambda x: (-x[1], x[2]))
        return candidates[0][0]
    
    def set_task(self, gpu_id: int, task_name: Optional[str]):
        """设置 GPU 当前任务"""
        with self._lock:
            if gpu_id in self.gpu_status:
                self.gpu_status[gpu_id].current_task = task_name
    
    def get_status_summary(self) -> str:
        """获取 GPU 状态摘要"""
        self.refresh_all()
        
        lines = ["=" * 60, "GPU 状态摘要", "=" * 60]
        for gpu_id, status in sorted(self.gpu_status.items()):
            task_info = f"[{status.current_task}]" if status.is_busy else "[空闲]"
            lines.append(
                f"GPU {gpu_id}: {status.free_memory_gb:.1f}GB 可用 / "
                f"{status.total_memory_gb:.1f}GB 总计 | "
                f"利用率 {status.utilization_percent:.0f}% | {task_info}"
            )
        lines.append("=" * 60)
        return "\n".join(lines)


class TaskScheduler:
    """任务调度器"""
    
    def __init__(self, workspace_dir: Optional[Path] = None):
        self.gpu_monitor = GPUMonitor()
        self.task_queue = queue.PriorityQueue()
        self.running_tasks: Dict[str, Task] = {}
        self.completed_tasks: List[Task] = []
        self._lock = threading.Lock()
        self._stop_event = threading.Event()
        self._worker_threads: List[threading.Thread] = []
        
        # 工作目录
        self.workspace_dir = workspace_dir or Path(__file__).parent.resolve()
        self.state_file = self.workspace_dir / ".gpu_scheduler_state.json"
        
        # 配置
        self.min_memory_gb = float(os.environ.get("GPU_MIN_MEMORY_GB", "8.0"))
        self.check_interval = float(os.environ.get("GPU_CHECK_INTERVAL", "5.0"))
        self.max_retries = int(os.environ.get("GPU_MAX_RETRIES", "3"))
    
    def submit_task(self, project_name: str, priority: int = 0) -> Task:
        """提交新任务"""
        task_id = f"{project_name}_{int(time.time() * 1000)}"
        task = Task(
            task_id=task_id,
            project_name=project_name,
            priority=priority
        )
        
        self.task_queue.put(task)
        print(f"任务已提交: {task_id} (项目: {project_name}, 优先级: {priority})")
        return task
    
    def _execute_task(self, task: Task, gpu_id: int) -> bool:
        """
        执行任务
        
        Args:
            task: 任务对象
            gpu_id: 分配的 GPU ID
            
        Returns:
            是否成功
        """
        task.assigned_gpu = gpu_id
        task.started_at = time.time()
        task.status = "running"
        
        self.gpu_monitor.set_task(gpu_id, task.project_name)
        
        with self._lock:
            self.running_tasks[task.task_id] = task
        
        print(f"\n[{datetime.now().strftime('%H:%M:%S')}] 开始任务: {task.task_id} (GPU {gpu_id})")
        
        try:
            # 设置环境变量
            env = os.environ.copy()
            env["CUDA_VISIBLE_DEVICES"] = str(gpu_id)
            env["PROJECT_NAME"] = task.project_name
            
            # 运行 resort_by_unixcoder.py
            cmd = [
                sys.executable,
                str(self.workspace_dir / "resort_by_unixcoder.py")
            ]
            
            result = subprocess.run(
                cmd,
                env=env,
                capture_output=True,
                text=True,
                timeout=3600  # 1 小时超时
            )
            
            if result.returncode == 0:
                task.status = "completed"
                task.completed_at = time.time()
                duration = task.completed_at - task.started_at
                print(f"[{datetime.now().strftime('%H:%M:%S')}] 任务完成: {task.task_id} (耗时: {duration:.1f}s)")
                return True
            else:
                task.status = "failed"
                task.error_message = result.stderr[-1000:] if result.stderr else "Unknown error"
                print(f"[{datetime.now().strftime('%H:%M:%S')}] 任务失败: {task.task_id}")
                print(f"错误信息: {task.error_message[:200]}...")
                return False
                
        except subprocess.TimeoutExpired:
            task.status = "failed"
            task.error_message = "任务超时"
            print(f"[{datetime.now().strftime('%H:%M:%S')}] 任务超时: {task.task_id}")
            return False
            
        except Exception as e:
            task.status = "failed"
            task.error_message = str(e)
            print(f"[{datetime.now().strftime('%H:%M:%S')}] 任务异常: {task.task_id} - {e}")
            return False
            
        finally:
            self.gpu_monitor.set_task(gpu_id, None)
            
            with self._lock:
                if task.task_id in self.running_tasks:
                    del self.running_tasks[task.task_id]
                self.completed_tasks.append(task)
    
    def _worker_loop(self, worker_id: int):
        """工作线程主循环"""
        print(f"[Worker {worker_id}] 启动")
        
        while not self._stop_event.is_set():
            try:
                # 尝试获取任务（非阻塞，超时 1 秒）
                try:
                    task = self.task_queue.get(timeout=1.0)
                except queue.Empty:
                    continue
                
                # 等待可用的 GPU
                gpu_id = None
                wait_count = 0
                while gpu_id is None and not self._stop_event.is_set():
                    gpu_id = self.gpu_monitor.get_best_gpu(self.min_memory_gb)
                    
                    if gpu_id is None:
                        if wait_count == 0:
                            print(f"[Worker {worker_id}] 等待 GPU 资源...")
                            print(self.gpu_monitor.get_status_summary())
                        wait_count += 1
                        time.sleep(self.check_interval)
                        
                        # 每分钟打印一次状态
                        if wait_count % 12 == 0:
                            print(f"[Worker {worker_id}] 仍在等待 GPU ({wait_count * self.check_interval:.0f}s)...")
                
                if self._stop_event.is_set():
                    # 将任务放回队列
                    self.task_queue.put(task)
                    break
                
                # 执行任务
                success = self._execute_task(task, gpu_id)
                
                if not success and task.status == "failed":
                    # 检查是否需要重试
                    retry_count = getattr(task, 'retry_count', 0)
                    if retry_count < self.max_retries and "OOM" in (task.error_message or ""):
                        task.retry_count = retry_count + 1
                        task.status = "pending"
                        task.error_message = None
                        print(f"[Worker {worker_id}] 任务将重试 ({task.retry_count}/{self.max_retries}): {task.task_id}")
                        time.sleep(30)  # 等待 GPU 内存释放
                        self.task_queue.put(task)
                
            except Exception as e:
                print(f"[Worker {worker_id}] 错误: {e}")
                import traceback
                traceback.print_exc()
        
        print(f"[Worker {worker_id}] 停止")
    
    def start(self, num_workers: int = None):
        """启动调度器"""
        if num_workers is None:
            num_workers = max(1, self.gpu_monitor.num_gpus)
        
        print(f"启动 GPU 任务调度器 (GPU 数量: {self.gpu_monitor.num_gpus}, Worker 数量: {num_workers})")
        print(self.gpu_monitor.get_status_summary())
        
        self._stop_event.clear()
        
        for i in range(num_workers):
            t = threading.Thread(target=self._worker_loop, args=(i,), daemon=True)
            t.start()
            self._worker_threads.append(t)
    
    def stop(self, wait: bool = True):
        """停止调度器"""
        print("正在停止调度器...")
        self._stop_event.set()
        
        if wait:
            for t in self._worker_threads:
                t.join(timeout=10)
        
        self._worker_threads.clear()
        print("调度器已停止")
    
    def wait_completion(self):
        """等待所有任务完成"""
        while not self.task_queue.empty() or self.running_tasks:
            time.sleep(1)
    
    def get_status(self) -> Dict:
        """获取调度器状态"""
        return {
            "gpu_count": self.gpu_monitor.num_gpus,
            "gpu_status": {
                gpu_id: {
                    "free_memory_gb": status.free_memory_gb,
                    "total_memory_gb": status.total_memory_gb,
                    "utilization_percent": status.utilization_percent,
                    "current_task": status.current_task,
                    "is_busy": status.is_busy
                }
                for gpu_id, status in self.gpu_monitor.gpu_status.items()
            },
            "pending_tasks": self.task_queue.qsize(),
            "running_tasks": list(self.running_tasks.keys()),
            "completed_tasks": len(self.completed_tasks)
        }


def run_server():
    """运行调度器服务"""
    scheduler = TaskScheduler()
    scheduler.start()
    
    print("\n调度器已启动，等待任务...")
    print("按 Ctrl+C 停止\n")
    
    try:
        while True:
            time.sleep(60)
            print(f"\n[{datetime.now().strftime('%H:%M:%S')}] 状态更新:")
            print(scheduler.gpu_monitor.get_status_summary())
            status = scheduler.get_status()
            print(f"待处理: {status['pending_tasks']}, 运行中: {len(status['running_tasks'])}, "
                  f"已完成: {status['completed_tasks']}")
    except KeyboardInterrupt:
        print("\n收到停止信号...")
        scheduler.stop()


def submit_task(project_name: str, priority: int = 0):
    """提交单个任务"""
    scheduler = TaskScheduler()
    task = scheduler.submit_task(project_name, priority)
    scheduler.start(num_workers=1)
    scheduler.wait_completion()
    scheduler.stop()
    
    if task.status == "completed":
        print(f"\n任务完成: {task.task_id}")
        return 0
    else:
        print(f"\n任务失败: {task.task_id}")
        print(f"错误: {task.error_message}")
        return 1


def show_status():
    """显示 GPU 状态"""
    monitor = GPUMonitor()
    print(monitor.get_status_summary())


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="GPU 任务队列调度器")
    parser.add_argument("--mode", choices=["server", "submit", "status"], default="status",
                        help="运行模式: server=服务模式, submit=提交任务, status=查看状态")
    parser.add_argument("--project", type=str, help="项目名称（submit 模式需要）")
    parser.add_argument("--priority", type=int, default=0, help="任务优先级")
    
    args = parser.parse_args()
    
    if args.mode == "server":
        run_server()
    elif args.mode == "submit":
        if not args.project:
            print("错误: submit 模式需要指定 --project")
            return 1
        return submit_task(args.project, args.priority)
    else:
        show_status()
        return 0


if __name__ == "__main__":
    sys.exit(main() or 0)


