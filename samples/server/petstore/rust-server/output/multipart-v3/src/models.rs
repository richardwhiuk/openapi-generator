#![allow(unused_qualifications)]

use validator::Validate;

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipartMultipleOptionsPostApplicationSlashJsonRequest(StringObject);

impl std::convert::From<StringObject> for MultipartMultipleOptionsPostApplicationSlashJsonRequest {
    fn from(x: StringObject) -> Self {
        MultipartMultipleOptionsPostApplicationSlashJsonRequest(x)
    }
}

impl std::convert::From<MultipartMultipleOptionsPostApplicationSlashJsonRequest> for StringObject {
    fn from(x: MultipartMultipleOptionsPostApplicationSlashJsonRequest) -> Self {
        x.0
    }
}

impl std::ops::Deref for MultipartMultipleOptionsPostApplicationSlashJsonRequest {
    type Target = StringObject;
    fn deref(&self) -> &StringObject {
        &self.0
    }
}

impl std::ops::DerefMut for MultipartMultipleOptionsPostApplicationSlashJsonRequest {
    fn deref_mut(&mut self) -> &mut StringObject {
        &mut self.0
    }
}

/// Converts the MultipartMultipleOptionsPostApplicationSlashJsonRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl ::std::string::ToString for MultipartMultipleOptionsPostApplicationSlashJsonRequest {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipartMultipleOptionsPostApplicationSlashJsonRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl ::std::str::FromStr for MultipartMultipleOptionsPostApplicationSlashJsonRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match std::str::FromStr::from_str(s) {
             std::result::Result::Ok(r) => std::result::Result::Ok(MultipartMultipleOptionsPostApplicationSlashJsonRequest(r)),
             std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {} to MultipartMultipleOptionsPostApplicationSlashJsonRequest: {:?}", s, e)),
        }
    }
}

// Methods for converting between header::IntoHeaderValue<MultipartMultipleOptionsPostApplicationSlashJsonRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipartMultipleOptionsPostApplicationSlashJsonRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipartMultipleOptionsPostApplicationSlashJsonRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipartMultipleOptionsPostApplicationSlashJsonRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipartMultipleOptionsPostApplicationSlashJsonRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipartMultipleOptionsPostApplicationSlashJsonRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipartMultipleOptionsPostApplicationSlashJsonRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipartMultipleOptionsPostApplicationSlashJsonRequest>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipartMultipleOptionsPostApplicationSlashJsonRequest>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipartMultipleOptionsPostApplicationSlashJsonRequest>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipartMultipleOptionsPostApplicationSlashJsonRequest> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipartMultipleOptionsPostApplicationSlashJsonRequest as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipartMultipleOptionsPostApplicationSlashJsonRequest - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipartMultipleOptionsPostMultipartSlashRelatedRequest(MultipartRelatedRequest);

impl std::convert::From<MultipartRelatedRequest> for MultipartMultipleOptionsPostMultipartSlashRelatedRequest {
    fn from(x: MultipartRelatedRequest) -> Self {
        MultipartMultipleOptionsPostMultipartSlashRelatedRequest(x)
    }
}

impl std::convert::From<MultipartMultipleOptionsPostMultipartSlashRelatedRequest> for MultipartRelatedRequest {
    fn from(x: MultipartMultipleOptionsPostMultipartSlashRelatedRequest) -> Self {
        x.0
    }
}

impl std::ops::Deref for MultipartMultipleOptionsPostMultipartSlashRelatedRequest {
    type Target = MultipartRelatedRequest;
    fn deref(&self) -> &MultipartRelatedRequest {
        &self.0
    }
}

impl std::ops::DerefMut for MultipartMultipleOptionsPostMultipartSlashRelatedRequest {
    fn deref_mut(&mut self) -> &mut MultipartRelatedRequest {
        &mut self.0
    }
}

/// Converts the MultipartMultipleOptionsPostMultipartSlashRelatedRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl ::std::string::ToString for MultipartMultipleOptionsPostMultipartSlashRelatedRequest {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipartMultipleOptionsPostMultipartSlashRelatedRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl ::std::str::FromStr for MultipartMultipleOptionsPostMultipartSlashRelatedRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match std::str::FromStr::from_str(s) {
             std::result::Result::Ok(r) => std::result::Result::Ok(MultipartMultipleOptionsPostMultipartSlashRelatedRequest(r)),
             std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {} to MultipartMultipleOptionsPostMultipartSlashRelatedRequest: {:?}", s, e)),
        }
    }
}

// Methods for converting between header::IntoHeaderValue<MultipartMultipleOptionsPostMultipartSlashRelatedRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipartMultipleOptionsPostMultipartSlashRelatedRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipartMultipleOptionsPostMultipartSlashRelatedRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipartMultipleOptionsPostMultipartSlashRelatedRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipartMultipleOptionsPostMultipartSlashRelatedRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipartMultipleOptionsPostMultipartSlashRelatedRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipartMultipleOptionsPostMultipartSlashRelatedRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipartMultipleOptionsPostMultipartSlashRelatedRequest>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipartMultipleOptionsPostMultipartSlashRelatedRequest>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipartMultipleOptionsPostMultipartSlashRelatedRequest>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipartMultipleOptionsPostMultipartSlashRelatedRequest> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipartMultipleOptionsPostMultipartSlashRelatedRequest as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipartMultipleOptionsPostMultipartSlashRelatedRequest - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipartMultipleResponsesGet200ApplicationSlashJsonResponse(StringObject);

impl std::convert::From<StringObject> for MultipartMultipleResponsesGet200ApplicationSlashJsonResponse {
    fn from(x: StringObject) -> Self {
        MultipartMultipleResponsesGet200ApplicationSlashJsonResponse(x)
    }
}

impl std::convert::From<MultipartMultipleResponsesGet200ApplicationSlashJsonResponse> for StringObject {
    fn from(x: MultipartMultipleResponsesGet200ApplicationSlashJsonResponse) -> Self {
        x.0
    }
}

impl std::ops::Deref for MultipartMultipleResponsesGet200ApplicationSlashJsonResponse {
    type Target = StringObject;
    fn deref(&self) -> &StringObject {
        &self.0
    }
}

impl std::ops::DerefMut for MultipartMultipleResponsesGet200ApplicationSlashJsonResponse {
    fn deref_mut(&mut self) -> &mut StringObject {
        &mut self.0
    }
}

/// Converts the MultipartMultipleResponsesGet200ApplicationSlashJsonResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl ::std::string::ToString for MultipartMultipleResponsesGet200ApplicationSlashJsonResponse {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipartMultipleResponsesGet200ApplicationSlashJsonResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl ::std::str::FromStr for MultipartMultipleResponsesGet200ApplicationSlashJsonResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match std::str::FromStr::from_str(s) {
             std::result::Result::Ok(r) => std::result::Result::Ok(MultipartMultipleResponsesGet200ApplicationSlashJsonResponse(r)),
             std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {} to MultipartMultipleResponsesGet200ApplicationSlashJsonResponse: {:?}", s, e)),
        }
    }
}

// Methods for converting between header::IntoHeaderValue<MultipartMultipleResponsesGet200ApplicationSlashJsonResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipartMultipleResponsesGet200ApplicationSlashJsonResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipartMultipleResponsesGet200ApplicationSlashJsonResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipartMultipleResponsesGet200ApplicationSlashJsonResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipartMultipleResponsesGet200ApplicationSlashJsonResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipartMultipleResponsesGet200ApplicationSlashJsonResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipartMultipleResponsesGet200ApplicationSlashJsonResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipartMultipleResponsesGet200ApplicationSlashJsonResponse>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipartMultipleResponsesGet200ApplicationSlashJsonResponse>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipartMultipleResponsesGet200ApplicationSlashJsonResponse>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipartMultipleResponsesGet200ApplicationSlashJsonResponse> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipartMultipleResponsesGet200ApplicationSlashJsonResponse as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipartMultipleResponsesGet200ApplicationSlashJsonResponse - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipartMultipleResponsesGet200MultipartSlashRelatedResponse(MultipartSingleResponseGet200Response);

impl std::convert::From<MultipartSingleResponseGet200Response> for MultipartMultipleResponsesGet200MultipartSlashRelatedResponse {
    fn from(x: MultipartSingleResponseGet200Response) -> Self {
        MultipartMultipleResponsesGet200MultipartSlashRelatedResponse(x)
    }
}

impl std::convert::From<MultipartMultipleResponsesGet200MultipartSlashRelatedResponse> for MultipartSingleResponseGet200Response {
    fn from(x: MultipartMultipleResponsesGet200MultipartSlashRelatedResponse) -> Self {
        x.0
    }
}

impl std::ops::Deref for MultipartMultipleResponsesGet200MultipartSlashRelatedResponse {
    type Target = MultipartSingleResponseGet200Response;
    fn deref(&self) -> &MultipartSingleResponseGet200Response {
        &self.0
    }
}

impl std::ops::DerefMut for MultipartMultipleResponsesGet200MultipartSlashRelatedResponse {
    fn deref_mut(&mut self) -> &mut MultipartSingleResponseGet200Response {
        &mut self.0
    }
}

/// Converts the MultipartMultipleResponsesGet200MultipartSlashRelatedResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl ::std::string::ToString for MultipartMultipleResponsesGet200MultipartSlashRelatedResponse {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipartMultipleResponsesGet200MultipartSlashRelatedResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl ::std::str::FromStr for MultipartMultipleResponsesGet200MultipartSlashRelatedResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match std::str::FromStr::from_str(s) {
             std::result::Result::Ok(r) => std::result::Result::Ok(MultipartMultipleResponsesGet200MultipartSlashRelatedResponse(r)),
             std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {} to MultipartMultipleResponsesGet200MultipartSlashRelatedResponse: {:?}", s, e)),
        }
    }
}

// Methods for converting between header::IntoHeaderValue<MultipartMultipleResponsesGet200MultipartSlashRelatedResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipartMultipleResponsesGet200MultipartSlashRelatedResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipartMultipleResponsesGet200MultipartSlashRelatedResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipartMultipleResponsesGet200MultipartSlashRelatedResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipartMultipleResponsesGet200MultipartSlashRelatedResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipartMultipleResponsesGet200MultipartSlashRelatedResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipartMultipleResponsesGet200MultipartSlashRelatedResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipartMultipleResponsesGet200MultipartSlashRelatedResponse>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipartMultipleResponsesGet200MultipartSlashRelatedResponse>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipartMultipleResponsesGet200MultipartSlashRelatedResponse>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipartMultipleResponsesGet200MultipartSlashRelatedResponse> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipartMultipleResponsesGet200MultipartSlashRelatedResponse as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipartMultipleResponsesGet200MultipartSlashRelatedResponse - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest(StringObject);

impl std::convert::From<StringObject> for MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest {
    fn from(x: StringObject) -> Self {
        MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest(x)
    }
}

impl std::convert::From<MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest> for StringObject {
    fn from(x: MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest) -> Self {
        x.0
    }
}

impl std::ops::Deref for MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest {
    type Target = StringObject;
    fn deref(&self) -> &StringObject {
        &self.0
    }
}

impl std::ops::DerefMut for MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest {
    fn deref_mut(&mut self) -> &mut StringObject {
        &mut self.0
    }
}

/// Converts the MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl ::std::string::ToString for MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl ::std::str::FromStr for MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match std::str::FromStr::from_str(s) {
             std::result::Result::Ok(r) => std::result::Result::Ok(MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest(r)),
             std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {} to MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest: {:?}", s, e)),
        }
    }
}

// Methods for converting between header::IntoHeaderValue<MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipartOptionalMultipleOptionsPostApplicationSlashJsonRequest - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest(MultipartRelatedRequest);

impl std::convert::From<MultipartRelatedRequest> for MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest {
    fn from(x: MultipartRelatedRequest) -> Self {
        MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest(x)
    }
}

impl std::convert::From<MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest> for MultipartRelatedRequest {
    fn from(x: MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest) -> Self {
        x.0
    }
}

impl std::ops::Deref for MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest {
    type Target = MultipartRelatedRequest;
    fn deref(&self) -> &MultipartRelatedRequest {
        &self.0
    }
}

impl std::ops::DerefMut for MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest {
    fn deref_mut(&mut self) -> &mut MultipartRelatedRequest {
        &mut self.0
    }
}

/// Converts the MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl ::std::string::ToString for MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl ::std::str::FromStr for MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match std::str::FromStr::from_str(s) {
             std::result::Result::Ok(r) => std::result::Result::Ok(MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest(r)),
             std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {} to MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest: {:?}", s, e)),
        }
    }
}

// Methods for converting between header::IntoHeaderValue<MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipartOptionalMultipleOptionsPostMultipartSlashRelatedRequest - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipartRelatedRequest {
    #[serde(rename = "object_field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_field: Option<models::MultipartRequestObjectField>,

    #[serde(rename = "optional_binary_field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub optional_binary_field: Option<swagger::ByteArray>,

    #[serde(rename = "required_binary_field")]
    pub required_binary_field: swagger::ByteArray,

}


impl MultipartRelatedRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(required_binary_field: swagger::ByteArray, ) -> MultipartRelatedRequest {
        MultipartRelatedRequest {
            object_field: None,
            optional_binary_field: None,
            required_binary_field,
        }
    }
}

/// Converts the MultipartRelatedRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MultipartRelatedRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping object_field in query parameter serialization

            // Skipping optional_binary_field in query parameter serialization
            // Skipping optional_binary_field in query parameter serialization

            // Skipping required_binary_field in query parameter serialization
            // Skipping required_binary_field in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipartRelatedRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MultipartRelatedRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub object_field: Vec<models::MultipartRequestObjectField>,
            pub optional_binary_field: Vec<swagger::ByteArray>,
            pub required_binary_field: Vec<swagger::ByteArray>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MultipartRelatedRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "object_field" => intermediate_rep.object_field.push(<models::MultipartRequestObjectField as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "optional_binary_field" => return std::result::Result::Err("Parsing binary data in this style is not supported in MultipartRelatedRequest".to_string()),
                    "required_binary_field" => return std::result::Result::Err("Parsing binary data in this style is not supported in MultipartRelatedRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MultipartRelatedRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MultipartRelatedRequest {
            object_field: intermediate_rep.object_field.into_iter().next(),
            optional_binary_field: intermediate_rep.optional_binary_field.into_iter().next(),
            required_binary_field: intermediate_rep.required_binary_field.into_iter().next().ok_or_else(|| "required_binary_field missing in MultipartRelatedRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MultipartRelatedRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipartRelatedRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipartRelatedRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipartRelatedRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipartRelatedRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipartRelatedRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipartRelatedRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipartRelatedRequest>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipartRelatedRequest>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipartRelatedRequest>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipartRelatedRequest> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipartRelatedRequest as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipartRelatedRequest - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipartRequestObjectField {
    #[serde(rename = "field_a")]
    pub field_a: String,

    #[serde(rename = "field_b")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub field_b: Option<Vec<String>>,

}


impl MultipartRequestObjectField {
    #[allow(clippy::new_without_default)]
    pub fn new(field_a: String, ) -> MultipartRequestObjectField {
        MultipartRequestObjectField {
            field_a,
            field_b: None,
        }
    }
}

/// Converts the MultipartRequestObjectField value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MultipartRequestObjectField {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("field_a".to_string()),
            Some(self.field_a.to_string()),


            self.field_b.as_ref().map(|field_b| {
                [
                    "field_b".to_string(),
                    field_b.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipartRequestObjectField value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MultipartRequestObjectField {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub field_a: Vec<String>,
            pub field_b: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MultipartRequestObjectField".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "field_a" => intermediate_rep.field_a.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "field_b" => return std::result::Result::Err("Parsing a container in this style is not supported in MultipartRequestObjectField".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MultipartRequestObjectField".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MultipartRequestObjectField {
            field_a: intermediate_rep.field_a.into_iter().next().ok_or_else(|| "field_a missing in MultipartRequestObjectField".to_string())?,
            field_b: intermediate_rep.field_b.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MultipartRequestObjectField> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipartRequestObjectField>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipartRequestObjectField>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipartRequestObjectField - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipartRequestObjectField> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipartRequestObjectField as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipartRequestObjectField - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipartRequestObjectField>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipartRequestObjectField>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipartRequestObjectField>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipartRequestObjectField> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipartRequestObjectField as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipartRequestObjectField - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipartSingleResponseGet200Response {
    #[serde(rename = "object_field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_field: Option<String>,

    #[serde(rename = "optional_binary_field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub optional_binary_field: Option<swagger::ByteArray>,

}


impl MultipartSingleResponseGet200Response {
    #[allow(clippy::new_without_default)]
    pub fn new() -> MultipartSingleResponseGet200Response {
        MultipartSingleResponseGet200Response {
            object_field: None,
            optional_binary_field: None,
        }
    }
}

/// Converts the MultipartSingleResponseGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MultipartSingleResponseGet200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.object_field.as_ref().map(|object_field| {
                [
                    "object_field".to_string(),
                    object_field.to_string(),
                ].join(",")
            }),

            // Skipping optional_binary_field in query parameter serialization
            // Skipping optional_binary_field in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipartSingleResponseGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MultipartSingleResponseGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub object_field: Vec<String>,
            pub optional_binary_field: Vec<swagger::ByteArray>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MultipartSingleResponseGet200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "object_field" => intermediate_rep.object_field.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "optional_binary_field" => return std::result::Result::Err("Parsing binary data in this style is not supported in MultipartSingleResponseGet200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MultipartSingleResponseGet200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MultipartSingleResponseGet200Response {
            object_field: intermediate_rep.object_field.into_iter().next(),
            optional_binary_field: intermediate_rep.optional_binary_field.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MultipartSingleResponseGet200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipartSingleResponseGet200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipartSingleResponseGet200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipartSingleResponseGet200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipartSingleResponseGet200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipartSingleResponseGet200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipartSingleResponseGet200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipartSingleResponseGet200Response>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipartSingleResponseGet200Response>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipartSingleResponseGet200Response>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipartSingleResponseGet200Response> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipartSingleResponseGet200Response as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipartSingleResponseGet200Response - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MultipleIdenticalMimeTypesPostRequest {
    #[serde(rename = "binary1")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub binary1: Option<swagger::ByteArray>,

    #[serde(rename = "binary2")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub binary2: Option<swagger::ByteArray>,

}


impl MultipleIdenticalMimeTypesPostRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> MultipleIdenticalMimeTypesPostRequest {
        MultipleIdenticalMimeTypesPostRequest {
            binary1: None,
            binary2: None,
        }
    }
}

/// Converts the MultipleIdenticalMimeTypesPostRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MultipleIdenticalMimeTypesPostRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping binary1 in query parameter serialization
            // Skipping binary1 in query parameter serialization

            // Skipping binary2 in query parameter serialization
            // Skipping binary2 in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MultipleIdenticalMimeTypesPostRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MultipleIdenticalMimeTypesPostRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub binary1: Vec<swagger::ByteArray>,
            pub binary2: Vec<swagger::ByteArray>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MultipleIdenticalMimeTypesPostRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "binary1" => return std::result::Result::Err("Parsing binary data in this style is not supported in MultipleIdenticalMimeTypesPostRequest".to_string()),
                    "binary2" => return std::result::Result::Err("Parsing binary data in this style is not supported in MultipleIdenticalMimeTypesPostRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MultipleIdenticalMimeTypesPostRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MultipleIdenticalMimeTypesPostRequest {
            binary1: intermediate_rep.binary1.into_iter().next(),
            binary2: intermediate_rep.binary2.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MultipleIdenticalMimeTypesPostRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MultipleIdenticalMimeTypesPostRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MultipleIdenticalMimeTypesPostRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MultipleIdenticalMimeTypesPostRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MultipleIdenticalMimeTypesPostRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MultipleIdenticalMimeTypesPostRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MultipleIdenticalMimeTypesPostRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<MultipleIdenticalMimeTypesPostRequest>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<MultipleIdenticalMimeTypesPostRequest>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<MultipleIdenticalMimeTypesPostRequest>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<MultipleIdenticalMimeTypesPostRequest> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <MultipleIdenticalMimeTypesPostRequest as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into MultipleIdenticalMimeTypesPostRequest - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StringObject(String);

impl std::convert::From<String> for StringObject {
    fn from(x: String) -> Self {
        StringObject(x)
    }
}

impl std::convert::From<StringObject> for String {
    fn from(x: StringObject) -> Self {
        x.0
    }
}

impl std::ops::Deref for StringObject {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for StringObject {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl std::string::ToString for StringObject {
    fn to_string(&self) -> String {
       self.0.clone()
    }
}

impl std::str::FromStr for StringObject {
    type Err = ::std::convert::Infallible;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(StringObject(x.to_owned()))
    }
}

// Methods for converting between header::IntoHeaderValue<StringObject> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StringObject>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StringObject>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StringObject - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StringObject> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StringObject as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StringObject - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Vec<StringObject>>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_values: header::IntoHeaderValue<Vec<StringObject>>) -> std::result::Result<Self, Self::Error> {
        let hdr_values : Vec<String> = hdr_values.0.into_iter().map(|hdr_value| {
            hdr_value.to_string()
        }).collect();

        match hyper::header::HeaderValue::from_str(&hdr_values.join(", ")) {
           std::result::Result::Ok(hdr_value) => std::result::Result::Ok(hdr_value),
           std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to convert {:?} into a header - {}",
               hdr_values, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Vec<StringObject>> {
    type Error = String;

    fn try_from(hdr_values: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_values.to_str() {
            std::result::Result::Ok(hdr_values) => {
                let hdr_values : std::vec::Vec<StringObject> = hdr_values
                .split(',')
                .filter_map(|hdr_value| match hdr_value.trim() {
                    "" => std::option::Option::None,
                    hdr_value => std::option::Option::Some({
                        match <StringObject as std::str::FromStr>::from_str(hdr_value) {
                            std::result::Result::Ok(value) => std::result::Result::Ok(value),
                            std::result::Result::Err(err) => std::result::Result::Err(
                                format!("Unable to convert header value '{}' into StringObject - {}",
                                    hdr_value, err))
                        }
                    })
                }).collect::<std::result::Result<std::vec::Vec<_>, String>>()?;

                std::result::Result::Ok(header::IntoHeaderValue(hdr_values))
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!("Unable to parse header: {:?} as a string - {}",
                hdr_values, e)),
        }
    }
}
