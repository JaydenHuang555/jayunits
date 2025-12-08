
use jayunits;

mod angular {
    use jayunits::{geom::{angle::angle_unit, distance::distance_unit}, motion::{acceleration::angular::{angular_acceleration_measure::AngularAcceleration, angular_acceleration_unit::{self, AngularAccelerationUnit}}, motion_unit::MotionUnit, velocity::angular::angular_velocity_unit}, time::time_unit};
    use jayunits::unit::Unit;
    use jayunits::measure::Measure;

    
    #[test]
    pub fn convert() {
        let input_unit = angular_acceleration_unit::RADIANS_PER_SECOND_PER_SECOND;
        let output_unit = &AngularAccelerationUnit::derive_units(angular_velocity_unit::DEGREES_PER_SECOND, time_unit::MINUTES);
        let acceleration = AngularAcceleration::from(1.0, input_unit);
        assert_eq!(
            acceleration.to(output_unit),
            1.0 / output_unit.get_scale_to_base()
        );
    }

}