mod distance {
    use jayunits::{
        geom::distance::{distance_measure::Distance, distance_unit},
        measure::Measure, unit_constants,
    };

    #[test]
    fn convert() {
        let input_unit = unit_constants::METERS;
        let output_unit = unit_constants::FEET;
        let input_value = 42.0;

        let distance = Distance::from(input_value, input_unit);

        assert_eq!(
            distance.to(output_unit),
            input_value / output_unit.get_scale_to_base()
        )
    }

    #[test]
    fn math() {
        let input1 = 39.9325;
        let input2 = 71.294;

        let operand1 = Distance::from(input1, unit_constants::METERS);
        let operand2 = Distance::from(input2, unit_constants::METERS);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);
    }
}

mod angle {
    use jayunits::{
        geom::angle::{angle_measure::Angle, angle_unit},
        measure::Measure, unit_constants,
    };

    #[test]
    fn convert() {
        let input_unit = unit_constants::RADIANS;
        let output_unit = unit_constants::ROTATIONS;
        let input_value = 31.0;

        let angle = Angle::from(input_value, input_unit);

        assert_eq!(
            angle.to(output_unit),
            input_value / output_unit.get_scale_to_base()
        )
    }

    #[test]
    fn math() {
        let input1 = 20.4921;
        let input2 = 85.392;

        let operand1 = Angle::from(input1, unit_constants::RADIANS);
        let operand2 = Angle::from(input2, unit_constants::RADIANS);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);
    }
}
