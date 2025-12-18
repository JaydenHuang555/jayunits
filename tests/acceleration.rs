mod angular {
    use jayunits::internal::measure::Measure;
    use jayunits::motion::{
        acceleration::angular::{
            angular_acceleration_measure::AngularAcceleration,
            angular_acceleration_unit::AngularAccelerationUnit,
        },
        motion_unit::MotionUnit,
    };
    use jayunits::constants::units;

    #[test]
    pub fn convert() {
        let input_unit = &units::RADIANS_PER_SECOND_PER_SECOND;
        let output_unit =
            &AngularAccelerationUnit::derive_units(&units::DEGREES_PER_SECOND, &units::MINUTES);
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

        let operand1 = AngularAcceleration::from(input1, &units::RADIANS_PER_SECOND_PER_SECOND);
        let operand2 = AngularAcceleration::from(input2, &units::RADIANS_PER_SECOND_PER_SECOND);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);
    }
}

mod linear {
    use jayunits::{
        internal::measure::Measure,
        motion::acceleration::linear::linear_acceleration_measure::LinearAcceleration, constants::units,
    };

    #[test]
    pub fn convert() {
        let input_unit = &units::METERS_PER_SECOND_PER_SECOND;
        let output_unit = &units::FEET_PER_SECOND_PER_SECOND;
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

        let operand1 = LinearAcceleration::from(input1, &units::METERS_PER_SECOND_PER_SECOND);
        let operand2 = LinearAcceleration::from(input2, &units::METERS_PER_SECOND_PER_SECOND);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);
    }
}
