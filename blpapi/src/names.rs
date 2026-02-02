use crate::name::Name;
use once_cell::sync::Lazy;

/// Consumer Warnings Names
pub static SLOW_CONSUMER_WARNING: Lazy<Name> = Lazy::new(|| Name::new("slowConsumerWarning"));
pub static SLOW_CONSUMER_WARNING_CLEARED: Lazy<Name> =
    Lazy::new(|| Name::new("slowConsumerWarningCleared"));

/// Data Loss Name
pub static DATA_LOSS: Lazy<Name> = Lazy::new(|| Name::new("dataLoss"));

/// Request Names
pub static REQUEST_TEMPLATE_AVAILABLE: Lazy<Name> =
    Lazy::new(|| Name::new("requestTemplateAvailable"));
pub static REQUEST_TEMPLATE_PENDING: Lazy<Name> = Lazy::new(|| Name::new("requestTemplatePending"));
pub static REQUEST_TEMPLATE_TERMINATED: Lazy<Name> =
    Lazy::new(|| Name::new("requestTemplateTerminated"));
pub static REQUEST_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("requestFailure"));

/// Subscription Names
pub static SUBSCRIPTION_TERMINATED: Lazy<Name> = Lazy::new(|| Name::new("subscriptionTerminated"));
pub static SUBSCRIPTION_STARTED: Lazy<Name> = Lazy::new(|| Name::new("subscriptionStarted"));
pub static SUBSCRIPTION_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("subscriptionFailure"));
pub static SUBSCRIPTION_STREAMS_ACTIVATED: Lazy<Name> =
    Lazy::new(|| Name::new("subscriptionStreamsActivated"));
pub static SUBSCRIPTION_STREAMS_DEACTIVATED: Lazy<Name> =
    Lazy::new(|| Name::new("subscriptionStreamsDeactivated"));

/// Token Names
pub static TOKEN_GENERATION_SUCCESS: Lazy<Name> = Lazy::new(|| Name::new("tokenGenerationSuccess"));
pub static TOKEN_GENERATION_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("tokenGenerationFailure"));

pub static SECURITY_DATA: Lazy<Name> = Lazy::new(|| Name::new("securityData"));
pub static SECURITY_NAME: Lazy<Name> = Lazy::new(|| Name::new("security"));
pub static SECURITY_ERROR: Lazy<Name> = Lazy::new(|| Name::new("securityError"));

pub static FIELD_DATA: Lazy<Name> = Lazy::new(|| Name::new("fieldData"));
pub static FIELD_DATA_ERROR: Lazy<Name> = Lazy::new(|| Name::new("fieldError"));
pub static FIELD_EID_DATA: Lazy<Name> = Lazy::new(|| Name::new("eidData"));

/// Session Names
pub static SESSION_STARTED: Lazy<Name> = Lazy::new(|| Name::new("sessionStarted"));
pub static SESSION_TERMINATED: Lazy<Name> = Lazy::new(|| Name::new("sessionTerminated"));
pub static SESSION_STARTUP_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("sessionStartupFailure"));
pub static SESSION_CONNECTION_UP: Lazy<Name> = Lazy::new(|| Name::new("sessionConnectionUp"));
pub static SESSION_CONNECTION_DOWN: Lazy<Name> = Lazy::new(|| Name::new("sessionConnectionDown"));

/// Service Names
pub static SERVICE_OPENED: Lazy<Name> = Lazy::new(|| Name::new("serviceOpened"));
pub static SERVICE_OPEN_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("serviceOpenFailure"));
pub static SERVICE_REGISTERED: Lazy<Name> = Lazy::new(|| Name::new("serviceRegistered"));
pub static SERVICE_REGISTER_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("serviceRegisterFailure"));
pub static SERVICE_DEREGISTERED: Lazy<Name> = Lazy::new(|| Name::new("serviceDerigistered"));
pub static SERVICE_UP: Lazy<Name> = Lazy::new(|| Name::new("serviceUp"));
pub static SERVICE_DOWN: Lazy<Name> = Lazy::new(|| Name::new("serviceDown"));
pub static SERVICE_AVAILABILITY_INFO: Lazy<Name> =
    Lazy::new(|| Name::new("serviceAvailabilityInfo"));

/// Resolution Names
pub static RESOLUTION_SUCCESS: Lazy<Name> = Lazy::new(|| Name::new("resolutionSuccess"));
pub static RESOLUTION_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("resolutionFailure"));

/// Topic Names
pub static TOPIC_SUBSCRIBED: Lazy<Name> = Lazy::new(|| Name::new("topicSubscribed"));
pub static TOPIC_UNSUBSCRIBED: Lazy<Name> = Lazy::new(|| Name::new("topicUnsubscribed"));
pub static TOPIC_RECAP: Lazy<Name> = Lazy::new(|| Name::new("topicRecap"));
pub static TOPIC_ACTIVATED: Lazy<Name> = Lazy::new(|| Name::new("topicActivated"));
pub static TOPIC_DEACTIVATED: Lazy<Name> = Lazy::new(|| Name::new("topicDeactivated"));
pub static TOPIC_CREATED: Lazy<Name> = Lazy::new(|| Name::new("topicCreated"));
pub static TOPIC_CREATE_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("topicCreateFailure"));
pub static TOPIC_DELETED: Lazy<Name> = Lazy::new(|| Name::new("topicDeleted"));
pub static TOPIC_RESUBSCRIBED: Lazy<Name> = Lazy::new(|| Name::new("topicResubscribed"));

/// Permission Names
pub static PERMISSION_REQUEST: Lazy<Name> = Lazy::new(|| Name::new("permissionRequest"));
pub static PERMISSION_RESPONSE: Lazy<Name> = Lazy::new(|| Name::new("permissionResponse"));

/// Authorization Names
pub static AUTHORIZATION_SUCCESS: Lazy<Name> = Lazy::new(|| Name::new("authorizationSuccess"));
pub static AUTHORIZATION_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("authorizationFailure"));
pub static AUTHORIZATION_REVOKED: Lazy<Name> = Lazy::new(|| Name::new("authorizationRevoked"));

/// Core Serives
pub static SECURITY: Lazy<Name> = Lazy::new(|| Name::new("security"));
pub static SECURITIES: Lazy<Name> = Lazy::new(|| Name::new("securities"));
pub static EVENT_TYPES: Lazy<Name> = Lazy::new(|| Name::new("eventTypes"));
pub static FIELDS_SEARCH: Lazy<Name> = Lazy::new(|| Name::new("searchSpec"));
pub static FIELDS_EXCLUDE: Lazy<Name> = Lazy::new(|| Name::new("exclude"));
pub static FIELDS_REQUEST_ID: Lazy<Name> = Lazy::new(|| Name::new("id"));
pub static FIELDS_NAME: Lazy<Name> = Lazy::new(|| Name::new("fields"));
pub static FIELDS_CATEGORY: Lazy<Name> = Lazy::new(|| Name::new("category"));
pub static FIELD_ID: Lazy<Name> = Lazy::new(|| Name::new("fieldId"));
pub static FIELD_TYPE: Lazy<Name> = Lazy::new(|| Name::new("fieldType"));
pub static FIELD_TYPE_DOCS: Lazy<Name> = Lazy::new(|| Name::new("returnFieldDocumentation"));
pub static OVERRIDES: Lazy<Name> = Lazy::new(|| Name::new("overrides"));
pub static VALUE: Lazy<Name> = Lazy::new(|| Name::new("value"));
pub static START_DATE_TIME: Lazy<Name> = Lazy::new(|| Name::new("startDateTime"));
pub static END_DATE_TIME: Lazy<Name> = Lazy::new(|| Name::new("endDateTime"));
pub static TICK_DATA: Lazy<Name> = Lazy::new(|| Name::new("tickData"));
pub static QUERY: Lazy<Name> = Lazy::new(|| Name::new("query"));
pub static YELLOW_KEY_FILTER: Lazy<Name> = Lazy::new(|| Name::new("yellowKeyFilter"));
pub static LANGUAGE_OVERRIDE: Lazy<Name> = Lazy::new(|| Name::new("languageOverride"));
pub static RESULTS: Lazy<Name> = Lazy::new(|| Name::new("results"));
pub static MAX_RESULTS: Lazy<Name> = Lazy::new(|| Name::new("maxResults"));
pub static PARTIAL_MATCH: Lazy<Name> = Lazy::new(|| Name::new("partialMatch"));
pub static BBG_ID: Lazy<Name> = Lazy::new(|| Name::new("bbgid"));
pub static TICKER: Lazy<Name> = Lazy::new(|| Name::new("ticker"));
pub static COUNTRY_CODE: Lazy<Name> = Lazy::new(|| Name::new("countryCode"));
pub static CURRENCY_CODE: Lazy<Name> = Lazy::new(|| Name::new("currencyCode"));
pub static CURVE_ID: Lazy<Name> = Lazy::new(|| Name::new("curveid"));
pub static SECURITY_TYPE: Lazy<Name> = Lazy::new(|| Name::new("type"));
pub static SECURITY_SUBTYPE: Lazy<Name> = Lazy::new(|| Name::new("subtype"));
