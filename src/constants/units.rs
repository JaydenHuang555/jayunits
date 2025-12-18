
use crate::{
    geom::{angle::angle_unit::AngleUnit, distance::distance_unit::DistanceUnit},
    motion::{
        acceleration::{
            angular::angular_acceleration_unit::AngularAccelerationUnit,
            linear::linear_acceleration_unit::LinearAccelerationUnit,
        },
        velocity::{
            angular::angular_velocity_unit::AngularVelocityUnit,
            linear::linear_velocity_unit::LinearVelocityUnit,
        },
    },
    time::time_unit::TimeUnit,
};

pub const RADIANS: AngleUnit = AngleUnit::from(1.0, "Radians", "rad");
pub const DEGREES: AngleUnit = AngleUnit::from(std::f64::consts::PI / 180.0, "Degrees", "°");
pub const GRADIANS: AngleUnit = AngleUnit::from(std::f64::consts::PI / 200.0, "Gradians", "gon");
pub const ARCMINUTES: AngleUnit =
    AngleUnit::from(std::f64::consts::PI / 10800.0, "Arcminutes", "′");
pub const ARCSECONDS: AngleUnit =
    AngleUnit::from(std::f64::consts::PI / 648_000.0, "Arcseconds", "″");
pub const REVOLUTIONS: AngleUnit =
    AngleUnit::from(2.0 * std::f64::consts::PI, "Revolutions", "rev");
pub const ROTATIONS: AngleUnit = AngleUnit::from(2.0 * std::f64::consts::PI, "Rotations", "rot");

pub const METERS: DistanceUnit = DistanceUnit::from(1.0, "Meters", "m");
pub const KILOMETERS: DistanceUnit = DistanceUnit::from(1000.0, "Kilometers", "km");
pub const CENTIMETERS: DistanceUnit = DistanceUnit::from(0.01, "Centimeters", "cm");
pub const MILLIMETERS: DistanceUnit = DistanceUnit::from(0.001, "Millimeters", "mm");
pub const MICROMETERS: DistanceUnit = DistanceUnit::from(0.000_001, "Micrometers", "µm");
pub const NANOMETERS: DistanceUnit = DistanceUnit::from(0.000_000_001, "Nanometers", "nm");

pub const MILES: DistanceUnit = DistanceUnit::from(1609.344, "Miles", "mi");
pub const YARDS: DistanceUnit = DistanceUnit::from(0.9144, "Yards", "yd");
pub const FEET: DistanceUnit = DistanceUnit::from(0.3048, "Feet", "ft");
pub const INCHES: DistanceUnit = DistanceUnit::from(0.0254, "Inches", "in");

pub const NAUTICAL_MILES: DistanceUnit = DistanceUnit::from(1852.0, "Nautical Miles", "nmi");

pub const ASTRONOMICAL_UNITS: DistanceUnit =
    DistanceUnit::from(149_597_870_700.0, "Astronomical Units", "AU");
pub const LIGHT_YEARS: DistanceUnit =
    DistanceUnit::from(9_460_730_472_580_800.0, "Light Years", "ly");
pub const PARSECS: &DistanceUnit = &DistanceUnit::from(30_856_775_814_913_672.0, "Parsecs", "pc");

pub const RADIANS_PER_SECOND_PER_SECOND: AngularAccelerationUnit =
    AngularAccelerationUnit::from(1.0, "Radians per Second per Second", "rad/s²");
pub const RADIANS_PER_MINUTE_PER_SECOND: AngularAccelerationUnit =
    AngularAccelerationUnit::from(60.0, "Radians per Minute per Second", "rad/min·s");
pub const DEGREES_PER_SECOND_PER_SECOND: AngularAccelerationUnit = AngularAccelerationUnit::from(
    std::f64::consts::PI / 180.0,
    "Degrees per Second per Second",
    "°/s²",
);
pub const GRADIANS_PER_SECOND_PER_SECOND: AngularAccelerationUnit = AngularAccelerationUnit::from(
    std::f64::consts::PI / 200.0,
    "Gradians per Second per Second",
    "gon/s²",
);
pub const ARCMINUTES_PER_SECOND_PER_SECOND: AngularAccelerationUnit =
    AngularAccelerationUnit::from(
        std::f64::consts::PI / 10800.0,
        "Arcminutes per Second per Second",
        "′/s²",
    );
pub const ARCSECONDS_PER_SECOND_PER_SECOND: AngularAccelerationUnit =
    AngularAccelerationUnit::from(
        std::f64::consts::PI / 648_000.0,
        "Arcseconds per Second per Second",
        "″/s²",
    );
pub const REVOLUTIONS_PER_SECOND_PER_SECOND: AngularAccelerationUnit =
    AngularAccelerationUnit::from(
        2.0 * std::f64::consts::PI,
        "Revolutions per Second per Second",
        "rev/s²",
    );
pub const ROTATIONS_PER_SECOND_PER_SECOND: AngularAccelerationUnit =
    AngularAccelerationUnit::from(
        2.0 * std::f64::consts::PI,
        "Rotations per Second per Second",
        "rot/s²",
    );

pub const METERS_PER_SECOND_PER_SECOND: LinearAccelerationUnit =
    LinearAccelerationUnit::from(1.0, "Meters per Second per Second", "m/s²");
pub const KILOMETERS_PER_HOUR_PER_SECOND: LinearAccelerationUnit =
    LinearAccelerationUnit::from(1000.0 / 3600.0, "Kilometers per Hour per Second", "km/h·s");
pub const MILES_PER_HOUR_PER_SECOND: LinearAccelerationUnit =
    LinearAccelerationUnit::from(1609.344 / 3600.0, "Miles per Hour per Second", "mph/s");
pub const FEET_PER_SECOND_PER_SECOND: LinearAccelerationUnit =
    LinearAccelerationUnit::from(0.3048, "Feet per Second per Second", "ft/s²");

pub const RADIANS_PER_SECOND: AngularVelocityUnit =
    AngularVelocityUnit::from(1.0, "Radians per Second", "rad / s");
pub const RADIANS_PER_MINUTE: AngularVelocityUnit =
    AngularVelocityUnit::from(60.0, "Radians per Minute", "rad / min");
pub const DEGREES_PER_SECOND: AngularVelocityUnit =
    AngularVelocityUnit::from(std::f64::consts::PI / 180.0, "Degrees per Second", "° / s");
pub const GRADIANS_PER_SECOND: AngularVelocityUnit = AngularVelocityUnit::from(
    std::f64::consts::PI / 200.0,
    "Gradians per Seconds",
    "gon / s",
);
pub const ARCMINUTES_PER_SECOND: AngularVelocityUnit = AngularVelocityUnit::from(
    std::f64::consts::PI / 10800.0,
    "Arcminutes per Seconds",
    "′",
);
pub const ARCSECONDS_PER_SECOND: AngularVelocityUnit = AngularVelocityUnit::from(
    std::f64::consts::PI / 648_000.0,
    "Arcseconds per Seconds",
    "″ / s",
);
pub const REVOLUTIONS_PER_SECOND: AngularVelocityUnit =
    AngularVelocityUnit::from(2.0 * std::f64::consts::PI, "Revolutions per s", "rev / s");
pub const ROTATIONS_PER_SECOND: AngularVelocityUnit =
    AngularVelocityUnit::from(2.0 * std::f64::consts::PI, "Rotations per s", "rot / s");

pub const METERS_PER_SECOND: LinearVelocityUnit =
    LinearVelocityUnit::from(1.0, "Meters per Second", "m/s");
pub const KILOMETERS_PER_HOUR: LinearVelocityUnit =
    LinearVelocityUnit::from(1000.0 / 3600.0, "Kilometers per Hour", "km/h");
pub const MILES_PER_HOUR: LinearVelocityUnit =
    LinearVelocityUnit::from(1609.344 / 3600.0, "Miles per Hour", "mph");
pub const FEET_PER_SECOND: LinearVelocityUnit =
    LinearVelocityUnit::from(0.3048, "Feet per Second", "ft/s");

pub const SECONDS: TimeUnit = TimeUnit::from(1.0, "Seconds", "s");
pub const MILLISECONDS: TimeUnit = TimeUnit::from(0.001, "Milliseconds", "ms");
pub const MICROSECONDS: TimeUnit = TimeUnit::from(0.000_001, "Microseconds", "µs");
pub const NANOSECONDS: TimeUnit = TimeUnit::from(0.000_000_001, "Nanoseconds", "ns");

pub const MINUTES: TimeUnit = TimeUnit::from(60.0, "Minutes", "min");
pub const HOURS: TimeUnit = TimeUnit::from(3600.0, "Hours", "h");
pub const DAYS: TimeUnit = TimeUnit::from(86_400.0, "Days", "d");
pub const WEEKS: TimeUnit = TimeUnit::from(604_800.0, "Weeks", "wk");
pub const YEARS: TimeUnit = TimeUnit::from(31_536_000.0, "Years", "yr");
