pub const TELEMETRY_MAX_LEN:              usize = 63;
pub const TELEMETRY_BYTES_PER_FIELD:      usize = 4;
pub const TELEMETRY_MAX_FIELDS:           usize = TELEMETRY_MAX_LEN - 1;
pub const TELEMETRY_MAX_BYTES:            usize = TELEMETRY_MAX_LEN * TELEMETRY_BYTES_PER_FIELD;

/* Mandatory fields: (plus: end marker(n-2), field count and CRC(n-1)) */
pub const TELEMETRY_MIN_BYTES:            usize = TELEMETRY_BYTES_PER_FIELD * 5;
pub const TELEMETRY_STANDARD_MIN_FIELDS:  usize = 10;
pub const TELEMETRY_FIELD_COUNT:          usize = 0;
pub const TELEMETRY_FIELD_PACKET_NUMBER:  usize = 1;
pub const TELEMETRY_FIELD_TYPE:           usize = 2;
pub const TELEMETRY_FIELD_FLAGS:          usize = 3;

/*        specific to packet type 0x77777777      */
pub const TELEMETRY_FIELD_DATE:           usize = 4;
pub const TELEMETRY_FIELD_TIME:           usize = 5;
pub const TELEMETRY_FIELD_MILLISECOND:    usize = 6;
pub const TELEMETRY_FIELD_MISSION_ID:     usize = 7;
pub const TELEMETRY_FIELD_LATITUDE:       usize = 8;
pub const TELEMETRY_FIELD_LONGITUDE:      usize = 9;
pub const TELEMETRY_FIELD_ALTITUDE:       usize = 10;
pub const TELEMETRY_FIELD_HEADING:        usize = 11;

pub const TELEMETRY_FIELD_VARIABLE:       usize = 4;
