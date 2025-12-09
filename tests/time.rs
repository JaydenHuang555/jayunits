mod time {
    use jayunits::{
        measure::Measure,
        time::{time_measure::Time, time_unit},
    };

    #[test]
    fn convert() {
        let input_unit = time_unit::SECONDS;
        let output_unit = time_unit::SECONDS;
        let input_value = 31.0;

        let angle = Time::from(input_value, input_unit);

        assert_eq!(
            angle.to(output_unit),
            input_value / output_unit.get_scale_to_base()
        )
    }

    #[test]
    fn math() {
        let input1 = 20.4921;
        let input2 = 85.392;

        let operand1 = Time::from(input1, time_unit::SECONDS);
        let operand2 = Time::from(input2, time_unit::SECONDS);

        assert_eq!((operand1 + operand2).get_base(), input1 + input2);
        assert_eq!((operand1 - operand2).get_base(), input1 - input2);
        assert_eq!((operand1 * operand2).get_base(), input1 * input2);
        assert_eq!((operand1 / operand2).get_base(), input1 / input2);
    }
}
