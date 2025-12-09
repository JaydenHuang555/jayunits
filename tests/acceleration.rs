mod angular {
    use jayunits::measure::Measure;
    use jayunits::{
        motion::{
            acceleration::angular::{
                angular_acceleration_measure::AngularAcceleration,
                angular_acceleration_unit::{self, AngularAccelerationUnit},
            },
            motion_unit::MotionUnit,
            velocity::angular::angular_velocity_unit,
        },
        time::time_unit,
    };

    #[test]
    pub fn convert() {
        let input_unit = angular_acceleration_unit::RADIANS_PER_SECOND_PER_SECOND;
        let output_unit = &AngularAccelerationUnit::derive_units(
            angular_velocity_unit::DEGREES_PER_SECOND,
            time_unit::MINUTES,
        );
        let acceleration = AngularAcceleration::from(1.0, input_unit);
        assert_eq!(
            acceleration.to(output_unit),
            1.0 / output_unit.get_scale_to_base()
        );
    }

    #[test]
    fn math() {
        let input1 = 48.112;
        let input2 = 90.34;

        let operand1 = AngularAcceleration::from(
            input1,
            angular_acceleration_unit::RADIANS_PER_SECOND_PER_SECOND,
        );
        let operand2 = AngularAcceleration::from(
            input2,
            angular_acceleration_unit::RADIANS_PER_SECOND_PER_SECOND,
        );

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);
    }
}

mod linear {
    use jayunits::{
        measure::Measure,
        motion::acceleration::linear::{
            linear_acceleration_measure::LinearAcceleration, linear_acceleration_unit,
        },
    };

    #[test]
    pub fn convert() {
        let input_unit = linear_acceleration_unit::METERS_PER_SECOND_PER_SECOND;
        let output_unit = linear_acceleration_unit::FEET_PER_SECOND_PER_SECOND;
        let acceleration = LinearAcceleration::from(1.0, input_unit);
        assert_eq!(
            acceleration.to(output_unit),
            1.0 / output_unit.get_scale_to_base()
        )
    }

    #[test]
    fn math() {
        let input1 = 32.1;
        let input2 = 63.32;

        let operand1 = LinearAcceleration::from(
            input1,
            linear_acceleration_unit::METERS_PER_SECOND_PER_SECOND,
        );
        let operand2 = LinearAcceleration::from(
            input2,
            linear_acceleration_unit::METERS_PER_SECOND_PER_SECOND,
        );

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);
    }
}
