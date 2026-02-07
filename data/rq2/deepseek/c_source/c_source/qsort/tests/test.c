#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

void quickSort(int arr[], int low, int high);

static int is_sorted(const int *arr, size_t n) {
  for (size_t i = 1; i < n; i++) {
    if (arr[i - 1] > arr[i]) {
      return 0;
    }
  }
  return 1;
}

static void assert_eq_arr(const int *got, const int *want, size_t n) {
  for (size_t i = 0; i < n; i++) {
    assert(got[i] == want[i]);
  }
}

static void run_case(const char *name, int *arr, size_t n) {
  if (n > 0) {
    quickSort(arr, 0, (int)(n - 1));
  }
  if (!is_sorted(arr, n)) {
    fprintf(stderr, "[fail] %s: array not sorted\n", name);
    abort();
  }
}

int main(void) {
  {
    int arr[] = {3, 1, 2};
    int want[] = {1, 2, 3};
    run_case("small", arr, sizeof(arr) / sizeof(arr[0]));
    assert_eq_arr(arr, want, sizeof(arr) / sizeof(arr[0]));
  }

  {
    int arr[] = {5, 4, 3, 2, 1};
    int want[] = {1, 2, 3, 4, 5};
    run_case("reverse", arr, sizeof(arr) / sizeof(arr[0]));
    assert_eq_arr(arr, want, sizeof(arr) / sizeof(arr[0]));
  }

  {
    int arr[] = {0, -1, 5, -3, 2};
    int want[] = {-3, -1, 0, 2, 5};
    run_case("negatives", arr, sizeof(arr) / sizeof(arr[0]));
    assert_eq_arr(arr, want, sizeof(arr) / sizeof(arr[0]));
  }

  {
    int arr[] = {7, 7, 7, 7};
    int want[] = {7, 7, 7, 7};
    run_case("duplicates", arr, sizeof(arr) / sizeof(arr[0]));
    assert_eq_arr(arr, want, sizeof(arr) / sizeof(arr[0]));
  }

  {
    enum { N = 1000 };
    int *arr = (int *)malloc(sizeof(int) * N);
    assert(arr);
    srand(1);
    for (int i = 0; i < N; i++) {
      arr[i] = (rand() % 2000) - 1000;
    }
    run_case("random_1000", arr, (size_t)N);
    free(arr);
  }

  printf("qsort tests: ok\n");
  return 0;
}

