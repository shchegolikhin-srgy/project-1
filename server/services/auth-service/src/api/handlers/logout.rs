use base64::prelude::*;
use axum::{
    http::StatusCode,
    Json,
};
use std::collections::HashMap;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use crate::models::login::{AuthResponse, LoginRequest};