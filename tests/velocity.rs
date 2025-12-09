mod linear {
    use jayunits::internal::measure::Measure;
    use jayunits::motion::velocity::linear::linear_velocity_measure::LinearVelocity;
    use jayunits::constants::units;

    #[test]
    fn convert() {
        let input_unit = units::METERS_PER_SECOND;
        let output_unit = units::FEET_PER_SECOND;
        let velocity = LinearVelocity::from(1.0, input_unit);

        assert_eq!(
            velocity.to(output_unit),
            1.0 / output_unit.get_scale_to_base()
        )
    }

    #[test]
    fn math() {
        let input1 = 31.53;
        let input2 = 12.53;

        let operand1 = LinearVelocity::from(input1, units::METERS_PER_SECOND);
        let operand2 = LinearVelocity::from(input2, units::METERS_PER_SECOND);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);
    }
}

mod angular {
    use jayunits::internal::measure::Measure;
    use jayunits::motion::velocity::angular::angular_velocity_measure::AngularVelocity;
    use jayunits::constants::units;

    #[test]
    fn convert() {
        let input_unt = units::RADIANS_PER_SECOND;
        let output_unit = units::ROTATIONS_PER_SECOND;
        let velocity = AngularVelocity::from(1.0, input_unt);
        assert_eq!(
            velocity.to(output_unit),
            1.0 / output_unit.get_scale_to_base()
        );
    }

    #[test]
    fn math() {
        let input1 = 31.53;
        let input2 = 12.53;

        let operand1 = AngularVelocity::from(input1, units::RADIANS_PER_SECOND);
        let operand2 = AngularVelocity::from(input2, units::RADIANS_PER_SECOND);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);
    }
}
