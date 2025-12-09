
mod distance {
    use jayunits::{geom::distance::{distance_measure::Distance, distance_unit}, measure::Measure};



    #[test]
    fn convert() {
        let input_unit = distance_unit::METERS;
        let output_unit = distance_unit::FEET;
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

        let operand1 = Distance::from(input1, distance_unit::METERS);
        let operand2 = Distance::from(input2, distance_unit::METERS);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);

    }

}

mod angle {
    use jayunits::{geom::angle::{angle_measure::Angle, angle_unit}, measure::Measure};


    #[test]
    fn convert() {
        let input_unit = angle_unit::RADIANS;
        let output_unit = angle_unit::ROTATIONS;
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

        let operand1 = Angle::from(input1, angle_unit::RADIANS);
        let operand2 = Angle::from(input2, angle_unit::RADIANS);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);

    }

}