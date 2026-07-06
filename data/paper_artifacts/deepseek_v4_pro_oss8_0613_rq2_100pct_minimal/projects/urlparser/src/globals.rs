//! Global and Static Variable Declarations (Scheme A: bindgen-truth static storage)
//!
//! - No safe wrappers (Mutex/RwLock).
//! - Types are derived from bindgen on the exact preprocessed `.i` TU.
//! - Storage is real Rust `static mut`, zero-initialized (C-like).
//! - NOTE: file-scope `static` (internal linkage) variables are emitted in each module file (Scheme B).

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use core::mem::MaybeUninit;
use crate::types::*;

// ==========================================
// Global Variables (top-level)
// ==========================================

// Source: url.h:54-74 populated with all 177 protocol strings
pub static URL_SCHEMES: [&[u8]; 177] = [
    b"aaa\0",
    b"aaas\0",
    b"about\0",
    b"acap\0",
    b"acct\0",
    b"adiumxtra\0",
    b"afp\0",
    b"afs\0",
    b"aim\0",
    b"apt\0",
    b"attachment\0",
    b"aw\0",
    b"beshare\0",
    b"bitcoin\0",
    b"bolo\0",
    b"callto\0",
    b"cap\0",
    b"chrome\0",
    b"crome-extension\0",
    b"com-evenbrite-attendee\0",
    b"cid\0",
    b"coap\0",
    b"coaps\0",
    b"content\0",
    b"crid\0",
    b"cvs\0",
    b"data\0",
    b"dav\0",
    b"dict\0",
    b"lna-playsingle\0",
    b"dln-playcontainer\0",
    b"dns\0",
    b"dtn\0",
    b"dvb\0",
    b"ed2k\0",
    b"facetime\0",
    b"fax\0",
    b"feed\0",
    b"file\0",
    b"finger\0",
    b"fish\0",
    b"ftp\0",
    b"geo\0",
    b"gg\0",
    b"git\0",
    b"gizmoproject\0",
    b"go\0",
    b"gopher\0",
    b"gtalk\0",
    b"h323\0",
    b"hcp\0",
    b"http\0",
    b"https\0",
    b"iax\0",
    b"icap\0",
    b"icon\0",
    b"im\0",
    b"imap\0",
    b"info\0",
    b"ipn\0",
    b"ipp\0",
    b"irc\0",
    b"irc6\0",
    b"ircs\0",
    b"iris\0",
    b"iris.beep\0",
    b"iris.xpc\0",
    b"iris.xpcs\0",
    b"iris.lws\0",
    b"itms\0",
    b"jabber\0",
    b"jar\0",
    b"jms\0",
    b"keyparc\0",
    b"lastfm\0",
    b"ldap\0",
    b"ldaps\0",
    b"magnet\0",
    b"mailserver\0",
    b"mailto\0",
    b"maps\0",
    b"market\0",
    b"message\0",
    b"mid\0",
    b"mms\0",
    b"modem\0",
    b"ms-help\0",
    b"mssettings-power\0",
    b"msnim\0",
    b"msrp\0",
    b"msrps\0",
    b"mtqp\0",
    b"mumble\0",
    b"mupdate\0",
    b"mvn\0",
    b"news\0",
    b"nfs\0",
    b"ni\0",
    b"nih\0",
    b"nntp\0",
    b"notes\0",
    b"oid\0",
    b"paquelocktoken\0",
    b"pack\0",
    b"palm\0",
    b"paparazzi\0",
    b"pkcs11\0",
    b"platform\0",
    b"pop\0",
    b"pres\0",
    b"prospero\0",
    b"proxy\0",
    b"psyc\0",
    b"query\0",
    b"reload\0",
    b"res\0",
    b"resource\0",
    b"rmi\0",
    b"rsync\0",
    b"rtmp\0",
    b"rtsp\0",
    b"secondlife\0",
    b"service\0",
    b"session\0",
    b"sftp\0",
    b"sgn\0",
    b"shttp\0",
    b"sieve\0",
    b"sip\0",
    b"sips\0",
    b"skype\0",
    b"smb\0",
    b"sms\0",
    b"snews\0",
    b"snmp\0",
    b"soap.beep\0",
    b"soap.beeps\0",
    b"soldat\0",
    b"spotify\0",
    b"ssh\0",
    b"steam\0",
    b"svn\0",
    b"tag\0",
    b"teamspeak\0",
    b"tel\0",
    b"telnet\0",
    b"tftp\0",
    b"things\0",
    b"thismessage\0",
    b"tn3270\0",
    b"tip\0",
    b"tv\0",
    b"udp\0",
    b"unreal\0",
    b"urn\0",
    b"ut2004\0",
    b"vemmi\0",
    b"ventrilo\0",
    b"videotex\0",
    b"view-source\0",
    b"wais\0",
    b"webcal\0",
    b"ws\0",
    b"wss\0",
    b"wtai\0",
    b"wyciwyg\0",
    b"xcon\0",
    b"xcon-userid\0",
    b"xfire\0",
    b"xmlrpc.beep\0",
    b"xmlrpc.beeps\0",
    b"xmpp\0",
    b"xri\0",
    b"ymsgr\0",
    b"javascript\0",
    b"jdbc\0",
    b"doi\0",
];


// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_auth

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_hash

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_host

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_hostname

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_path

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_pathname

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_port

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_protocol

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_query

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_get_search

// Source: bindgen missing (declaration omitted; see globals_generation_report.json)
// MISSING: url_parse