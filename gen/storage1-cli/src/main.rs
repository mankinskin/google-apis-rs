// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_storage1 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate, FlowType};
use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n> {
    opt: ArgMatches<'n>,
    hub: api::Storage<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _bucket_access_controls_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bucket_access_controls().delete(opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _bucket_access_controls_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bucket_access_controls().get(opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _bucket_access_controls_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-id" => Some(("entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.project-number" => Some(("projectTeam.projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.team" => Some(("projectTeam.team", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "id", "kind", "project-number", "project-team", "role", "self-link", "team"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BucketAccessControl = json::value::from_value(object).unwrap();
        let mut call = self.hub.bucket_access_controls().insert(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _bucket_access_controls_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.bucket_access_controls().list(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _bucket_access_controls_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-id" => Some(("entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.project-number" => Some(("projectTeam.projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.team" => Some(("projectTeam.team", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "id", "kind", "project-number", "project-team", "role", "self-link", "team"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BucketAccessControl = json::value::from_value(object).unwrap();
        let mut call = self.hub.bucket_access_controls().patch(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _bucket_access_controls_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-id" => Some(("entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.project-number" => Some(("projectTeam.projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.team" => Some(("projectTeam.team", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "id", "kind", "project-number", "project-team", "role", "self-link", "team"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BucketAccessControl = json::value::from_value(object).unwrap();
        let mut call = self.hub.bucket_access_controls().update(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _buckets_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buckets().delete(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-metageneration-not-match", "user-project", "provisional-user-project", "if-metageneration-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _buckets_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buckets().get(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-metageneration-match", "if-metageneration-not-match", "user-project", "provisional-user-project", "projection"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _buckets_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buckets().get_iam_policy(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _buckets_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "owner.entity-id" => Some(("owner.entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity" => Some(("owner.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-event-based-hold" => Some(("defaultEventBasedHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billing.requester-pays" => Some(("billing.requesterPays", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "encryption.default-kms-key-name" => Some(("encryption.defaultKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website.not-found-page" => Some(("website.notFoundPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website.main-page-suffix" => Some(("website.mainPageSuffix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-created" => Some(("timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metageneration" => Some(("metageneration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-number" => Some(("projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging.log-object-prefix" => Some(("logging.logObjectPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging.log-bucket" => Some(("logging.logBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-type" => Some(("locationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-policy.effective-time" => Some(("retentionPolicy.effectiveTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-policy.is-locked" => Some(("retentionPolicy.isLocked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "retention-policy.retention-period" => Some(("retentionPolicy.retentionPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-class" => Some(("storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iam-configuration.uniform-bucket-level-access.enabled" => Some(("iamConfiguration.uniformBucketLevelAccess.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iam-configuration.uniform-bucket-level-access.locked-time" => Some(("iamConfiguration.uniformBucketLevelAccess.lockedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iam-configuration.bucket-policy-only.enabled" => Some(("iamConfiguration.bucketPolicyOnly.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iam-configuration.bucket-policy-only.locked-time" => Some(("iamConfiguration.bucketPolicyOnly.lockedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "versioning.enabled" => Some(("versioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing", "bucket-policy-only", "default-event-based-hold", "default-kms-key-name", "effective-time", "enabled", "encryption", "entity", "entity-id", "etag", "iam-configuration", "id", "is-locked", "kind", "labels", "location", "location-type", "locked-time", "log-bucket", "log-object-prefix", "logging", "main-page-suffix", "metageneration", "name", "not-found-page", "owner", "project-number", "requester-pays", "retention-period", "retention-policy", "self-link", "storage-class", "time-created", "uniform-bucket-level-access", "updated", "versioning", "website"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Bucket = json::value::from_value(object).unwrap();
        let mut call = self.hub.buckets().insert(request, opt.value_of("project").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-default-object-acl" => {
                    call = call.predefined_default_object_acl(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["predefined-acl", "user-project", "provisional-user-project", "projection", "predefined-default-object-acl"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _buckets_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buckets().list(opt.value_of("project").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "prefix" => {
                    call = call.prefix(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["projection", "user-project", "max-results", "page-token", "prefix", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _buckets_lock_retention_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buckets().lock_retention_policy(opt.value_of("bucket").unwrap_or(""), opt.value_of("if-metageneration-match").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _buckets_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "owner.entity-id" => Some(("owner.entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity" => Some(("owner.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-event-based-hold" => Some(("defaultEventBasedHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billing.requester-pays" => Some(("billing.requesterPays", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "encryption.default-kms-key-name" => Some(("encryption.defaultKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website.not-found-page" => Some(("website.notFoundPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website.main-page-suffix" => Some(("website.mainPageSuffix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-created" => Some(("timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metageneration" => Some(("metageneration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-number" => Some(("projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging.log-object-prefix" => Some(("logging.logObjectPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging.log-bucket" => Some(("logging.logBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-type" => Some(("locationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-policy.effective-time" => Some(("retentionPolicy.effectiveTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-policy.is-locked" => Some(("retentionPolicy.isLocked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "retention-policy.retention-period" => Some(("retentionPolicy.retentionPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-class" => Some(("storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iam-configuration.uniform-bucket-level-access.enabled" => Some(("iamConfiguration.uniformBucketLevelAccess.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iam-configuration.uniform-bucket-level-access.locked-time" => Some(("iamConfiguration.uniformBucketLevelAccess.lockedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iam-configuration.bucket-policy-only.enabled" => Some(("iamConfiguration.bucketPolicyOnly.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iam-configuration.bucket-policy-only.locked-time" => Some(("iamConfiguration.bucketPolicyOnly.lockedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "versioning.enabled" => Some(("versioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing", "bucket-policy-only", "default-event-based-hold", "default-kms-key-name", "effective-time", "enabled", "encryption", "entity", "entity-id", "etag", "iam-configuration", "id", "is-locked", "kind", "labels", "location", "location-type", "locked-time", "log-bucket", "log-object-prefix", "logging", "main-page-suffix", "metageneration", "name", "not-found-page", "owner", "project-number", "requester-pays", "retention-period", "retention-policy", "self-link", "storage-class", "time-created", "uniform-bucket-level-access", "updated", "versioning", "website"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Bucket = json::value::from_value(object).unwrap();
        let mut call = self.hub.buckets().patch(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-default-object-acl" => {
                    call = call.predefined_default_object_acl(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["projection", "if-metageneration-not-match", "user-project", "predefined-default-object-acl", "predefined-acl", "provisional-user-project", "if-metageneration-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _buckets_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "kind", "resource-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Policy = json::value::from_value(object).unwrap();
        let mut call = self.hub.buckets().set_iam_policy(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _buckets_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.buckets().test_iam_permissions(opt.value_of("bucket").unwrap_or(""), &opt.values_of("permissions").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _buckets_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "owner.entity-id" => Some(("owner.entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity" => Some(("owner.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-event-based-hold" => Some(("defaultEventBasedHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billing.requester-pays" => Some(("billing.requesterPays", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "encryption.default-kms-key-name" => Some(("encryption.defaultKmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website.not-found-page" => Some(("website.notFoundPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website.main-page-suffix" => Some(("website.mainPageSuffix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-created" => Some(("timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metageneration" => Some(("metageneration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-number" => Some(("projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging.log-object-prefix" => Some(("logging.logObjectPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging.log-bucket" => Some(("logging.logBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-type" => Some(("locationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-policy.effective-time" => Some(("retentionPolicy.effectiveTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-policy.is-locked" => Some(("retentionPolicy.isLocked", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "retention-policy.retention-period" => Some(("retentionPolicy.retentionPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-class" => Some(("storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iam-configuration.uniform-bucket-level-access.enabled" => Some(("iamConfiguration.uniformBucketLevelAccess.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iam-configuration.uniform-bucket-level-access.locked-time" => Some(("iamConfiguration.uniformBucketLevelAccess.lockedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iam-configuration.bucket-policy-only.enabled" => Some(("iamConfiguration.bucketPolicyOnly.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iam-configuration.bucket-policy-only.locked-time" => Some(("iamConfiguration.bucketPolicyOnly.lockedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "versioning.enabled" => Some(("versioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing", "bucket-policy-only", "default-event-based-hold", "default-kms-key-name", "effective-time", "enabled", "encryption", "entity", "entity-id", "etag", "iam-configuration", "id", "is-locked", "kind", "labels", "location", "location-type", "locked-time", "log-bucket", "log-object-prefix", "logging", "main-page-suffix", "metageneration", "name", "not-found-page", "owner", "project-number", "requester-pays", "retention-period", "retention-policy", "self-link", "storage-class", "time-created", "uniform-bucket-level-access", "updated", "versioning", "website"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Bucket = json::value::from_value(object).unwrap();
        let mut call = self.hub.buckets().update(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-default-object-acl" => {
                    call = call.predefined_default_object_acl(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["projection", "if-metageneration-not-match", "user-project", "predefined-default-object-acl", "predefined-acl", "provisional-user-project", "if-metageneration-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _channels_stop(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "payload" => Some(("payload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "params" => Some(("params", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "expiration" => Some(("expiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address" => Some(("address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.channels().stop(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _default_object_access_controls_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.default_object_access_controls().delete(opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _default_object_access_controls_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.default_object_access_controls().get(opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _default_object_access_controls_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object" => Some(("object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-id" => Some(("entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.project-number" => Some(("projectTeam.projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.team" => Some(("projectTeam.team", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ObjectAccessControl = json::value::from_value(object).unwrap();
        let mut call = self.hub.default_object_access_controls().insert(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _default_object_access_controls_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.default_object_access_controls().list(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-metageneration-not-match", "user-project", "provisional-user-project", "if-metageneration-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _default_object_access_controls_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object" => Some(("object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-id" => Some(("entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.project-number" => Some(("projectTeam.projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.team" => Some(("projectTeam.team", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ObjectAccessControl = json::value::from_value(object).unwrap();
        let mut call = self.hub.default_object_access_controls().patch(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _default_object_access_controls_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object" => Some(("object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-id" => Some(("entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.project-number" => Some(("projectTeam.projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.team" => Some(("projectTeam.team", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ObjectAccessControl = json::value::from_value(object).unwrap();
        let mut call = self.hub.default_object_access_controls().update(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _notifications_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.notifications().delete(opt.value_of("bucket").unwrap_or(""), opt.value_of("notification").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _notifications_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.notifications().get(opt.value_of("bucket").unwrap_or(""), opt.value_of("notification").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _notifications_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-attributes" => Some(("custom_attributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "topic" => Some(("topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-types" => Some(("event_types", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "payload-format" => Some(("payload_format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object-name-prefix" => Some(("object_name_prefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["custom-attributes", "etag", "event-types", "id", "kind", "object-name-prefix", "payload-format", "self-link", "topic"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Notification = json::value::from_value(object).unwrap();
        let mut call = self.hub.notifications().insert(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _notifications_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.notifications().list(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _object_access_controls_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.object_access_controls().delete(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["generation", "user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _object_access_controls_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.object_access_controls().get(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["generation", "user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _object_access_controls_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object" => Some(("object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-id" => Some(("entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.project-number" => Some(("projectTeam.projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.team" => Some(("projectTeam.team", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ObjectAccessControl = json::value::from_value(object).unwrap();
        let mut call = self.hub.object_access_controls().insert(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["generation", "user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _object_access_controls_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.object_access_controls().list(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["generation", "user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _object_access_controls_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object" => Some(("object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-id" => Some(("entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.project-number" => Some(("projectTeam.projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.team" => Some(("projectTeam.team", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ObjectAccessControl = json::value::from_value(object).unwrap();
        let mut call = self.hub.object_access_controls().patch(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["generation", "user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _object_access_controls_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "domain" => Some(("domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object" => Some(("object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity" => Some(("entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-id" => Some(("entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.project-number" => Some(("projectTeam.projectNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-team.team" => Some(("projectTeam.team", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "domain", "email", "entity", "entity-id", "etag", "generation", "id", "kind", "object", "project-number", "project-team", "role", "self-link", "team"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ObjectAccessControl = json::value::from_value(object).unwrap();
        let mut call = self.hub.object_access_controls().update(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""), opt.value_of("entity").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["generation", "user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_compose(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.self-link" => Some(("destination.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.generation" => Some(("destination.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.component-count" => Some(("destination.componentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "destination.media-link" => Some(("destination.mediaLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.kms-key-name" => Some(("destination.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.owner.entity-id" => Some(("destination.owner.entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.owner.entity" => Some(("destination.owner.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.cache-control" => Some(("destination.cacheControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.id" => Some(("destination.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.size" => Some(("destination.size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.time-deleted" => Some(("destination.timeDeleted", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.time-created" => Some(("destination.timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.md5-hash" => Some(("destination.md5Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.crc32c" => Some(("destination.crc32c", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.etag" => Some(("destination.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.temporary-hold" => Some(("destination.temporaryHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "destination.metadata" => Some(("destination.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "destination.updated" => Some(("destination.updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.content-type" => Some(("destination.contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.content-language" => Some(("destination.contentLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.customer-encryption.encryption-algorithm" => Some(("destination.customerEncryption.encryptionAlgorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.customer-encryption.key-sha256" => Some(("destination.customerEncryption.keySha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.metageneration" => Some(("destination.metageneration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.time-storage-class-updated" => Some(("destination.timeStorageClassUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.retention-expiration-time" => Some(("destination.retentionExpirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.event-based-hold" => Some(("destination.eventBasedHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "destination.kind" => Some(("destination.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.name" => Some(("destination.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.bucket" => Some(("destination.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.content-encoding" => Some(("destination.contentEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.storage-class" => Some(("destination.storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "destination.content-disposition" => Some(("destination.contentDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "customer-encryption", "destination", "encryption-algorithm", "entity", "entity-id", "etag", "event-based-hold", "generation", "id", "key-sha256", "kind", "kms-key-name", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "retention-expiration-time", "self-link", "size", "storage-class", "temporary-hold", "time-created", "time-deleted", "time-storage-class-updated", "updated"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ComposeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.objects().compose(request, opt.value_of("destination-bucket").unwrap_or(""), opt.value_of("destination-object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "kms-key-name" => {
                    call = call.kms_key_name(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "destination-predefined-acl" => {
                    call = call.destination_predefined_acl(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-metageneration-match", "user-project", "destination-predefined-acl", "kms-key-name", "if-generation-match", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_copy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "component-count" => Some(("componentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "media-link" => Some(("mediaLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-name" => Some(("kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity-id" => Some(("owner.entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity" => Some(("owner.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cache-control" => Some(("cacheControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-deleted" => Some(("timeDeleted", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-created" => Some(("timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-hash" => Some(("md5Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "crc32c" => Some(("crc32c", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "temporary-hold" => Some(("temporaryHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata" => Some(("metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-language" => Some(("contentLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.encryption-algorithm" => Some(("customerEncryption.encryptionAlgorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.key-sha256" => Some(("customerEncryption.keySha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metageneration" => Some(("metageneration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-storage-class-updated" => Some(("timeStorageClassUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-expiration-time" => Some(("retentionExpirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-based-hold" => Some(("eventBasedHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-encoding" => Some(("contentEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-class" => Some(("storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-disposition" => Some(("contentDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "customer-encryption", "encryption-algorithm", "entity", "entity-id", "etag", "event-based-hold", "generation", "id", "key-sha256", "kind", "kms-key-name", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "retention-expiration-time", "self-link", "size", "storage-class", "temporary-hold", "time-created", "time-deleted", "time-storage-class-updated", "updated"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Object = json::value::from_value(object).unwrap();
        let mut call = self.hub.objects().copy(request, opt.value_of("source-bucket").unwrap_or(""), opt.value_of("source-object").unwrap_or(""), opt.value_of("destination-bucket").unwrap_or(""), opt.value_of("destination-object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "source-generation" => {
                    call = call.source_generation(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "if-source-metageneration-not-match" => {
                    call = call.if_source_metageneration_not_match(value.unwrap_or(""));
                },
                "if-source-metageneration-match" => {
                    call = call.if_source_metageneration_match(value.unwrap_or(""));
                },
                "if-source-generation-not-match" => {
                    call = call.if_source_generation_not_match(value.unwrap_or(""));
                },
                "if-source-generation-match" => {
                    call = call.if_source_generation_match(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "destination-predefined-acl" => {
                    call = call.destination_predefined_acl(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-source-generation-match", "projection", "if-source-metageneration-not-match", "if-metageneration-not-match", "user-project", "source-generation", "destination-predefined-acl", "if-source-generation-not-match", "if-source-metageneration-match", "if-generation-match", "provisional-user-project", "if-metageneration-match", "if-generation-not-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.objects().delete(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-generation-not-match", "generation", "if-metageneration-not-match", "user-project", "if-generation-match", "provisional-user-project", "if-metageneration-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _objects_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.objects().get(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            if key == "alt" && value.unwrap_or("unset") == "media" {
                                download_mode = true;
                            }
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["projection", "generation", "if-metageneration-match", "user-project", "if-generation-match", "provisional-user-project", "if-metageneration-not-match", "if-generation-not-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    ostream.flush().unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _objects_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.objects().get_iam_policy(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["generation", "user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "component-count" => Some(("componentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "media-link" => Some(("mediaLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-name" => Some(("kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity-id" => Some(("owner.entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity" => Some(("owner.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cache-control" => Some(("cacheControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-deleted" => Some(("timeDeleted", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-created" => Some(("timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-hash" => Some(("md5Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "crc32c" => Some(("crc32c", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "temporary-hold" => Some(("temporaryHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata" => Some(("metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-language" => Some(("contentLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.encryption-algorithm" => Some(("customerEncryption.encryptionAlgorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.key-sha256" => Some(("customerEncryption.keySha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metageneration" => Some(("metageneration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-storage-class-updated" => Some(("timeStorageClassUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-expiration-time" => Some(("retentionExpirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-based-hold" => Some(("eventBasedHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-encoding" => Some(("contentEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-class" => Some(("storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-disposition" => Some(("contentDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "customer-encryption", "encryption-algorithm", "entity", "entity-id", "etag", "event-based-hold", "generation", "id", "key-sha256", "kind", "kms-key-name", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "retention-expiration-time", "self-link", "size", "storage-class", "temporary-hold", "time-created", "time-deleted", "time-storage-class-updated", "updated"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Object = json::value::from_value(object).unwrap();
        let mut call = self.hub.objects().insert(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                "kms-key-name" => {
                    call = call.kms_key_name(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "content-encoding" => {
                    call = call.content_encoding(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-generation-match", "projection", "if-metageneration-not-match", "user-project", "content-encoding", "if-generation-not-match", "kms-key-name", "predefined-acl", "provisional-user-project", "if-metageneration-match", "name"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.objects().list(opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "versions" => {
                    call = call.versions(arg_from_str(value.unwrap_or("false"), err, "versions", "boolean"));
                },
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "prefix" => {
                    call = call.prefix(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-trailing-delimiter" => {
                    call = call.include_trailing_delimiter(arg_from_str(value.unwrap_or("false"), err, "include-trailing-delimiter", "boolean"));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["projection", "versions", "user-project", "delimiter", "max-results", "page-token", "prefix", "include-trailing-delimiter", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "component-count" => Some(("componentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "media-link" => Some(("mediaLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-name" => Some(("kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity-id" => Some(("owner.entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity" => Some(("owner.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cache-control" => Some(("cacheControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-deleted" => Some(("timeDeleted", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-created" => Some(("timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-hash" => Some(("md5Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "crc32c" => Some(("crc32c", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "temporary-hold" => Some(("temporaryHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata" => Some(("metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-language" => Some(("contentLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.encryption-algorithm" => Some(("customerEncryption.encryptionAlgorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.key-sha256" => Some(("customerEncryption.keySha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metageneration" => Some(("metageneration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-storage-class-updated" => Some(("timeStorageClassUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-expiration-time" => Some(("retentionExpirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-based-hold" => Some(("eventBasedHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-encoding" => Some(("contentEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-class" => Some(("storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-disposition" => Some(("contentDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "customer-encryption", "encryption-algorithm", "entity", "entity-id", "etag", "event-based-hold", "generation", "id", "key-sha256", "kind", "kms-key-name", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "retention-expiration-time", "self-link", "size", "storage-class", "temporary-hold", "time-created", "time-deleted", "time-storage-class-updated", "updated"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Object = json::value::from_value(object).unwrap();
        let mut call = self.hub.objects().patch(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-generation-match", "projection", "generation", "if-metageneration-match", "user-project", "predefined-acl", "provisional-user-project", "if-metageneration-not-match", "if-generation-not-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_rewrite(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "component-count" => Some(("componentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "media-link" => Some(("mediaLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-name" => Some(("kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity-id" => Some(("owner.entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity" => Some(("owner.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cache-control" => Some(("cacheControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-deleted" => Some(("timeDeleted", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-created" => Some(("timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-hash" => Some(("md5Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "crc32c" => Some(("crc32c", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "temporary-hold" => Some(("temporaryHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata" => Some(("metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-language" => Some(("contentLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.encryption-algorithm" => Some(("customerEncryption.encryptionAlgorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.key-sha256" => Some(("customerEncryption.keySha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metageneration" => Some(("metageneration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-storage-class-updated" => Some(("timeStorageClassUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-expiration-time" => Some(("retentionExpirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-based-hold" => Some(("eventBasedHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-encoding" => Some(("contentEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-class" => Some(("storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-disposition" => Some(("contentDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "customer-encryption", "encryption-algorithm", "entity", "entity-id", "etag", "event-based-hold", "generation", "id", "key-sha256", "kind", "kms-key-name", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "retention-expiration-time", "self-link", "size", "storage-class", "temporary-hold", "time-created", "time-deleted", "time-storage-class-updated", "updated"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Object = json::value::from_value(object).unwrap();
        let mut call = self.hub.objects().rewrite(request, opt.value_of("source-bucket").unwrap_or(""), opt.value_of("source-object").unwrap_or(""), opt.value_of("destination-bucket").unwrap_or(""), opt.value_of("destination-object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "source-generation" => {
                    call = call.source_generation(value.unwrap_or(""));
                },
                "rewrite-token" => {
                    call = call.rewrite_token(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "max-bytes-rewritten-per-call" => {
                    call = call.max_bytes_rewritten_per_call(value.unwrap_or(""));
                },
                "if-source-metageneration-not-match" => {
                    call = call.if_source_metageneration_not_match(value.unwrap_or(""));
                },
                "if-source-metageneration-match" => {
                    call = call.if_source_metageneration_match(value.unwrap_or(""));
                },
                "if-source-generation-not-match" => {
                    call = call.if_source_generation_not_match(value.unwrap_or(""));
                },
                "if-source-generation-match" => {
                    call = call.if_source_generation_match(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "destination-predefined-acl" => {
                    call = call.destination_predefined_acl(value.unwrap_or(""));
                },
                "destination-kms-key-name" => {
                    call = call.destination_kms_key_name(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-source-generation-match", "if-generation-match", "projection", "if-source-metageneration-not-match", "destination-kms-key-name", "if-metageneration-not-match", "user-project", "source-generation", "max-bytes-rewritten-per-call", "if-source-generation-not-match", "destination-predefined-acl", "if-source-metageneration-match", "rewrite-token", "provisional-user-project", "if-metageneration-match", "if-generation-not-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "kind", "resource-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Policy = json::value::from_value(object).unwrap();
        let mut call = self.hub.objects().set_iam_policy(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["generation", "user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.objects().test_iam_permissions(opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""), &opt.values_of("permissions").map(|i|i.collect()).unwrap_or(Vec::new()).iter().map(|&v| v.to_string()).collect::<Vec<String>>());
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["generation", "user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "generation" => Some(("generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "component-count" => Some(("componentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "media-link" => Some(("mediaLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-name" => Some(("kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity-id" => Some(("owner.entityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.entity" => Some(("owner.entity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cache-control" => Some(("cacheControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "size" => Some(("size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-deleted" => Some(("timeDeleted", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-created" => Some(("timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "md5-hash" => Some(("md5Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "crc32c" => Some(("crc32c", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "temporary-hold" => Some(("temporaryHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata" => Some(("metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-language" => Some(("contentLanguage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.encryption-algorithm" => Some(("customerEncryption.encryptionAlgorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "customer-encryption.key-sha256" => Some(("customerEncryption.keySha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metageneration" => Some(("metageneration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-storage-class-updated" => Some(("timeStorageClassUpdated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "retention-expiration-time" => Some(("retentionExpirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-based-hold" => Some(("eventBasedHold", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bucket" => Some(("bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-encoding" => Some(("contentEncoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-class" => Some(("storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-disposition" => Some(("contentDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bucket", "cache-control", "component-count", "content-disposition", "content-encoding", "content-language", "content-type", "crc32c", "customer-encryption", "encryption-algorithm", "entity", "entity-id", "etag", "event-based-hold", "generation", "id", "key-sha256", "kind", "kms-key-name", "md5-hash", "media-link", "metadata", "metageneration", "name", "owner", "retention-expiration-time", "self-link", "size", "storage-class", "temporary-hold", "time-created", "time-deleted", "time-storage-class-updated", "updated"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Object = json::value::from_value(object).unwrap();
        let mut call = self.hub.objects().update(request, opt.value_of("bucket").unwrap_or(""), opt.value_of("object").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["if-generation-match", "projection", "generation", "if-metageneration-match", "user-project", "predefined-acl", "provisional-user-project", "if-metageneration-not-match", "if-generation-not-match"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _objects_watch_all(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-id" => Some(("resourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "payload" => Some(("payload", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "params" => Some(("params", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "expiration" => Some(("expiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address" => Some(("address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "expiration", "id", "kind", "params", "payload", "resource-id", "resource-uri", "token", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.objects().watch_all(request, opt.value_of("bucket").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "versions" => {
                    call = call.versions(arg_from_str(value.unwrap_or("false"), err, "versions", "boolean"));
                },
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "prefix" => {
                    call = call.prefix(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-trailing-delimiter" => {
                    call = call.include_trailing_delimiter(arg_from_str(value.unwrap_or("false"), err, "include-trailing-delimiter", "boolean"));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["projection", "versions", "user-project", "delimiter", "max-results", "page-token", "prefix", "include-trailing-delimiter", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_hmac_keys_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().hmac_keys_create(opt.value_of("project-id").unwrap_or(""), opt.value_of("service-account-email").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_hmac_keys_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().hmac_keys_delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("access-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _projects_hmac_keys_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().hmac_keys_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("access-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_hmac_keys_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().hmac_keys_list(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "show-deleted-keys" => {
                    call = call.show_deleted_keys(arg_from_str(value.unwrap_or("false"), err, "show-deleted-keys", "boolean"));
                },
                "service-account-email" => {
                    call = call.service_account_email(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["show-deleted-keys", "user-project", "service-account-email", "max-results", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_hmac_keys_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-created" => Some(("timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "access-id" => Some(("accessId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account-email" => Some(("serviceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-id", "etag", "id", "kind", "project-id", "self-link", "service-account-email", "state", "time-created", "updated"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::HmacKeyMetadata = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().hmac_keys_update(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("access-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_service_account_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().service_account_get(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-project" => {
                    call = call.user_project(value.unwrap_or(""));
                },
                "provisional-user-project" => {
                    call = call.provisional_user_project(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["user-project", "provisional-user-project"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("bucket-access-controls", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._bucket_access_controls_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._bucket_access_controls_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._bucket_access_controls_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._bucket_access_controls_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._bucket_access_controls_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._bucket_access_controls_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("bucket-access-controls".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("buckets", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._buckets_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._buckets_get(opt, dry_run, &mut err);
                    },
                    ("get-iam-policy", Some(opt)) => {
                        call_result = self._buckets_get_iam_policy(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._buckets_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._buckets_list(opt, dry_run, &mut err);
                    },
                    ("lock-retention-policy", Some(opt)) => {
                        call_result = self._buckets_lock_retention_policy(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._buckets_patch(opt, dry_run, &mut err);
                    },
                    ("set-iam-policy", Some(opt)) => {
                        call_result = self._buckets_set_iam_policy(opt, dry_run, &mut err);
                    },
                    ("test-iam-permissions", Some(opt)) => {
                        call_result = self._buckets_test_iam_permissions(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._buckets_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("buckets".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("channels", Some(opt)) => {
                match opt.subcommand() {
                    ("stop", Some(opt)) => {
                        call_result = self._channels_stop(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("channels".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("default-object-access-controls", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._default_object_access_controls_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._default_object_access_controls_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._default_object_access_controls_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._default_object_access_controls_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._default_object_access_controls_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._default_object_access_controls_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("default-object-access-controls".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("notifications", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._notifications_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._notifications_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._notifications_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._notifications_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("notifications".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("object-access-controls", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._object_access_controls_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._object_access_controls_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._object_access_controls_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._object_access_controls_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._object_access_controls_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._object_access_controls_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("object-access-controls".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("objects", Some(opt)) => {
                match opt.subcommand() {
                    ("compose", Some(opt)) => {
                        call_result = self._objects_compose(opt, dry_run, &mut err);
                    },
                    ("copy", Some(opt)) => {
                        call_result = self._objects_copy(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._objects_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._objects_get(opt, dry_run, &mut err);
                    },
                    ("get-iam-policy", Some(opt)) => {
                        call_result = self._objects_get_iam_policy(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._objects_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._objects_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._objects_patch(opt, dry_run, &mut err);
                    },
                    ("rewrite", Some(opt)) => {
                        call_result = self._objects_rewrite(opt, dry_run, &mut err);
                    },
                    ("set-iam-policy", Some(opt)) => {
                        call_result = self._objects_set_iam_policy(opt, dry_run, &mut err);
                    },
                    ("test-iam-permissions", Some(opt)) => {
                        call_result = self._objects_test_iam_permissions(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._objects_update(opt, dry_run, &mut err);
                    },
                    ("watch-all", Some(opt)) => {
                        call_result = self._objects_watch_all(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("objects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("hmac-keys-create", Some(opt)) => {
                        call_result = self._projects_hmac_keys_create(opt, dry_run, &mut err);
                    },
                    ("hmac-keys-delete", Some(opt)) => {
                        call_result = self._projects_hmac_keys_delete(opt, dry_run, &mut err);
                    },
                    ("hmac-keys-get", Some(opt)) => {
                        call_result = self._projects_hmac_keys_get(opt, dry_run, &mut err);
                    },
                    ("hmac-keys-list", Some(opt)) => {
                        call_result = self._projects_hmac_keys_list(opt, dry_run, &mut err);
                    },
                    ("hmac-keys-update", Some(opt)) => {
                        call_result = self._projects_hmac_keys_update(opt, dry_run, &mut err);
                    },
                    ("service-account-get", Some(opt)) => {
                        call_result = self._projects_service_account_get(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "storage1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                                                })
                                        } else {
                                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
                                        },
                                        JsonTokenStorage {
                                          program_name: "storage1",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                    })
            } else {
                hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
            };
        let engine = Engine {
            opt: opt,
            hub: api::Storage::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
                ]
        };

        match engine._doit(true) {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false) {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

fn main() {
    let mut exit_status = 0i32;
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("bucket-access-controls", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Permanently deletes the ACL entry for the specified entity on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns the ACL entry for the specified entity on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",
                    Some(r##"Creates a new ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_insert",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Retrieves ACL entries on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_list",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Patches an ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",
                    Some(r##"Updates an ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/bucket-access-controls_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("buckets", "methods: 'delete', 'get', 'get-iam-policy', 'insert', 'list', 'lock-retention-policy', 'patch', 'set-iam-policy', 'test-iam-permissions' and 'update'", vec![
            ("delete",
                    Some(r##"Permanently deletes an empty bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns metadata for the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get-iam-policy",
                    Some(r##"Returns an IAM policy for the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_get-iam-policy",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",
                    Some(r##"Creates a new bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"A valid API project identifier."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Retrieves a list of buckets for a given project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"A valid API project identifier."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("lock-retention-policy",
                    Some(r##"Locks retention policy on a bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_lock-retention-policy",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"if-metageneration-match"##),
                     None,
                     Some(r##"Makes the operation conditional on whether bucket's current metageneration matches the given value."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Patches a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("set-iam-policy",
                    Some(r##"Updates an IAM policy for the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_set-iam-policy",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("test-iam-permissions",
                    Some(r##"Tests a set of permissions on the given bucket to see which, if any, are held by the caller."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_test-iam-permissions",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permissions"##),
                     None,
                     Some(r##"Permissions to test."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",
                    Some(r##"Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/buckets_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("channels", "methods: 'stop'", vec![
            ("stop",
                    Some(r##"Stop watching resources through this channel"##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/channels_stop",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ]),
        
        ("default-object-access-controls", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Permanently deletes the default object ACL entry for the specified entity on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns the default object ACL entry for the specified entity on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",
                    Some(r##"Creates a new default object ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_insert",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Retrieves default object ACL entries on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_list",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Patches a default object ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",
                    Some(r##"Updates a default object ACL entry on the specified bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/default-object-access-controls_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("notifications", "methods: 'delete', 'get', 'insert' and 'list'", vec![
            ("delete",
                    Some(r##"Permanently deletes a notification subscription."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/notifications_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"The parent bucket of the notification."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"notification"##),
                     None,
                     Some(r##"ID of the notification to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"View a notification configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/notifications_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"The parent bucket of the notification."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"notification"##),
                     None,
                     Some(r##"Notification ID"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",
                    Some(r##"Creates a notification subscription for a given bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/notifications_insert",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"The parent bucket of the notification."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Retrieves a list of notification subscriptions for a given bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/notifications_list",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a Google Cloud Storage bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("object-access-controls", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Permanently deletes the ACL entry for the specified entity on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns the ACL entry for the specified entity on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",
                    Some(r##"Creates a new ACL entry on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_insert",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Retrieves ACL entries on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_list",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Patches an ACL entry on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",
                    Some(r##"Updates an ACL entry on the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/object-access-controls_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of a bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"entity"##),
                     None,
                     Some(r##"The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("objects", "methods: 'compose', 'copy', 'delete', 'get', 'get-iam-policy', 'insert', 'list', 'patch', 'rewrite', 'set-iam-policy', 'test-iam-permissions', 'update' and 'watch-all'", vec![
            ("compose",
                    Some(r##"Concatenates a list of existing objects into a new object in the same bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_compose",
                  vec![
                    (Some(r##"destination-bucket"##),
                     None,
                     Some(r##"Name of the bucket containing the source objects. The destination object is stored in this bucket."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-object"##),
                     None,
                     Some(r##"Name of the new object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("copy",
                    Some(r##"Copies a source object to a destination object. Optionally overrides metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_copy",
                  vec![
                    (Some(r##"source-bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to find the source object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"source-object"##),
                     None,
                     Some(r##"Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-object"##),
                     None,
                     Some(r##"Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("delete",
                    Some(r##"Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_delete",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Retrieves an object or its metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_get",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get-iam-policy",
                    Some(r##"Returns an IAM policy for the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_get-iam-policy",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insert",
                    Some(r##"Stores a new object and metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_insert",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Retrieves a list of objects matching the criteria."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_list",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to look for objects."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Patches an object's metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_patch",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("rewrite",
                    Some(r##"Rewrites a source object to a destination object. Optionally overrides metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_rewrite",
                  vec![
                    (Some(r##"source-bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to find the source object."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"source-object"##),
                     None,
                     Some(r##"Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"destination-object"##),
                     None,
                     Some(r##"Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("set-iam-policy",
                    Some(r##"Updates an IAM policy for the specified object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_set-iam-policy",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("test-iam-permissions",
                    Some(r##"Tests a set of permissions on the given object to see which, if any, are held by the caller."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_test-iam-permissions",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permissions"##),
                     None,
                     Some(r##"Permissions to test."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("update",
                    Some(r##"Updates an object's metadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_update",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which the object resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"object"##),
                     None,
                     Some(r##"Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("watch-all",
                    Some(r##"Watch for changes on all objects in a bucket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/objects_watch-all",
                  vec![
                    (Some(r##"bucket"##),
                     None,
                     Some(r##"Name of the bucket in which to look for objects."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("projects", "methods: 'hmac-keys-create', 'hmac-keys-delete', 'hmac-keys-get', 'hmac-keys-list', 'hmac-keys-update' and 'service-account-get'", vec![
            ("hmac-keys-create",
                    Some(r##"Creates a new HMAC key for the specified service account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/projects_hmac-keys-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID owning the service account."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"service-account-email"##),
                     None,
                     Some(r##"Email address of the service account."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("hmac-keys-delete",
                    Some(r##"Deletes an HMAC key."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/projects_hmac-keys-delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID owning the requested key"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"access-id"##),
                     None,
                     Some(r##"Name of the HMAC key to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("hmac-keys-get",
                    Some(r##"Retrieves an HMAC key's metadata"##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/projects_hmac-keys-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID owning the service account of the requested key."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"access-id"##),
                     None,
                     Some(r##"Name of the HMAC key."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("hmac-keys-list",
                    Some(r##"Retrieves a list of HMAC keys matching the criteria."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/projects_hmac-keys-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Name of the project in which to look for HMAC keys."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("hmac-keys-update",
                    Some(r##"Updates the state of an HMAC key. See the HMAC Key resource descriptor for valid states."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/projects_hmac-keys-update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID owning the service account of the updated key."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"access-id"##),
                     None,
                     Some(r##"Name of the HMAC key being updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("service-account-get",
                    Some(r##"Get the email address of this project's Google Cloud Storage service account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storage1_cli/projects_service-account-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("storage1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.11+20190624")
           .about("Stores and retrieves potentially large, immutable data objects.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_storage1_cli")
           .arg(Arg::with_name("url")
                   .long("scope")
                   .help("Specify the authentication a method should be executed in. Each scope requires the user to grant this application permission to use it.If unset, it defaults to the shortest scope url for a particular method.")
                   .multiple(true)
                   .takes_value(true))
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
                   .multiple(false)
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Output all server communication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false))
           .arg(Arg::with_name("debug-auth")
                   .long("debug-auth")
                   .help("Output all communication related to authentication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::with_name(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::with_name(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
                   scmd = scmd.after_help(url_info);
           
                   for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                       let arg_name_str =
                           match (arg_name, flag) {
                                   (&Some(an), _       ) => an,
                                   (_        , &Some(f)) => f,
                                    _                    => unreachable!(),
                            };
                       let mut arg = Arg::with_name(arg_name_str)
                                         .empty_values(false);
                       if let &Some(short_flag) = flag {
                           arg = arg.short(short_flag);
                       }
                       if let &Some(desc) = desc {
                           arg = arg.help(desc);
                       }
                       if arg_name.is_some() && flag.is_some() {
                           arg = arg.takes_value(true);
                       }
                       if let &Some(required) = required {
                           arg = arg.required(required);
                       }
                       if let &Some(multi) = multi {
                           arg = arg.multiple(multi);
                       }
                       if arg_name_str == "mode" {
                           arg = arg.number_of_values(2);
                           arg = arg.value_names(&upload_value_names);
           
                           scmd = scmd.arg(Arg::with_name("mime")
                                               .short("m")
                                               .requires("mode")
                                               .required(false)
                                               .help("The file's mime time, like 'application/octet-stream'")
                                               .takes_value(true));
                       }
                       scmd = scmd.arg(arg);
                   }
                   mcmd = mcmd.subcommand(scmd);
               }
               app = app.subcommand(mcmd);
           }
           
        let matches = app.get_matches();

    let debug = matches.is_present("debug");
    match Engine::new(matches) {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}