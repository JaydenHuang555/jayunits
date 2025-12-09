
mod linear {
    use jayunits::motion::velocity::linear::{linear_velocity_measure::LinearVelocity, linear_velocity_unit};
    use jayunits::measure::Measure;


    #[test]
    fn convert() {
        let input_unit = linear_velocity_unit::METERS_PER_SECOND;
        let output_unit = linear_velocity_unit::FEET_PER_SECOND;
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

        let operand1 = LinearVelocity::from(input1, linear_velocity_unit::METERS_PER_SECOND);
        let operand2 = LinearVelocity::from(input2, linear_velocity_unit::METERS_PER_SECOND);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);

    }

}

mod angular {
    use jayunits::motion::velocity::angular::{angular_velocity_measure::AngularVelocity, angular_velocity_unit};
    use jayunits::measure::Measure;


    #[test]
    fn convert() {
        let input_unt = angular_velocity_unit::RADIANS_PER_SECOND;
        let output_unit = angular_velocity_unit::ROTATIONS_PER_SECOND;
        let velocity = AngularVelocity::from(1.0, input_unt);
        assert_eq!(velocity.to(output_unit), 1.0 / output_unit.get_scale_to_base());
    }

    #[test]
    fn math() {
        let input1 = 31.53;
        let input2 = 12.53;

        let operand1 = AngularVelocity::from(input1, angular_velocity_unit::RADIANS_PER_SECOND);
        let operand2 = AngularVelocity::from(input2, angular_velocity_unit::RADIANS_PER_SECOND);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);

    }

}