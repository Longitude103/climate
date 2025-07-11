pub enum Units {
    Celsius,
    Fahrenheit,
    Millimeters,
    Centimeters,
    Meters,
    Kilometers,
    Inches,
    Feet,
    Yards,
    Miles,
    Langley,
    MegaJoulesPerSquareMeter,
    WattsPerSquareMeter,
    Pascals,
    KiloPascals,
    Degrees,
    Radians,
    MetersPerSecond,
    MilesPerHour,
    Acres,
    Hectares,
    SquareFeet,
    SquareMeters,
    Percent,
}

impl Units {
    pub fn to_abbreviation(&self) -> &str {
        match self {
            Units::Celsius => "°C",
            Units::Fahrenheit => "°F",
            Units::Millimeters => "mm",
            Units::Centimeters => "cm",
            Units::Meters => "m",
            Units::Kilometers => "km",
            Units::Inches => "in",
            Units::Feet => "ft",
            Units::Yards => "yd",
            Units::Miles => "mi",
            Units::Langley => "L",
            Units::MegaJoulesPerSquareMeter => "MJ/m²",
            Units::WattsPerSquareMeter => "W/m²",
            Units::Pascals => "Pa",
            Units::KiloPascals => "kPa",
            Units::Degrees => "°",
            Units::Radians => "rad",
            Units::MetersPerSecond => "m/s",
            Units::MilesPerHour => "mph",
            Units::Acres => "acres",
            Units::Hectares => "ha",
            Units::SquareFeet => "ft²",
            Units::SquareMeters => "m²",
            Units::Percent => "%",
        }
    }

    pub fn from_abbreviation(abbreviation: &str) -> Result<Units, String> {
        match abbreviation {
            "°C" => Ok(Units::Celsius),
            "C" => Ok(Units::Celsius),
            "c" => Ok(Units::Celsius),
            "°F" => Ok(Units::Fahrenheit),
            "F" => Ok(Units::Fahrenheit),
            "f" => Ok(Units::Fahrenheit),
            "mm" => Ok(Units::Millimeters),
            "cm" => Ok(Units::Centimeters),
            "m" => Ok(Units::Meters),
            "km" => Ok(Units::Kilometers),
            "KM" => Ok(Units::Kilometers),
            "in" => Ok(Units::Inches),
            "ft" => Ok(Units::Feet),
            "yd" => Ok(Units::Yards),
            "mi" => Ok(Units::Miles),
            "L" => Ok(Units::Langley),
            "MJ/m²" => Ok(Units::MegaJoulesPerSquareMeter),
            "mj/m²" => Ok(Units::MegaJoulesPerSquareMeter),
            "mj/m2" => Ok(Units::MegaJoulesPerSquareMeter),
            "mj/m^2" => Ok(Units::MegaJoulesPerSquareMeter),
            "mJ/m^2" => Ok(Units::MegaJoulesPerSquareMeter),
            "W/m²" => Ok(Units::WattsPerSquareMeter),
            "w/m²" => Ok(Units::WattsPerSquareMeter),
            "W/m-2" => Ok(Units::WattsPerSquareMeter),
            "w/m-2" => Ok(Units::WattsPerSquareMeter),
            "Pa" => Ok(Units::Pascals),
            "pa" => Ok(Units::Pascals),
            "kpa" => Ok(Units::KiloPascals),
            "kPa" => Ok(Units::KiloPascals),
            "KPA" => Ok(Units::KiloPascals),
            "KPa" => Ok(Units::KiloPascals),
            "°" => Ok(Units::Degrees),
            "deg" => Ok(Units::Degrees),
            "rad" => Ok(Units::Radians),
            "m/s" => Ok(Units::MetersPerSecond),
            "mph" => Ok(Units::MilesPerHour),
            "acres" => Ok(Units::Acres),
            "ha" => Ok(Units::Hectares),
            "ft²" => Ok(Units::SquareFeet),
            "sq ft" => Ok(Units::SquareFeet),
            "ft2" => Ok(Units::SquareFeet),
            "m²" => Ok(Units::SquareMeters),
            "sq m" => Ok(Units::SquareMeters),
            "m2" => Ok(Units::SquareMeters),
            "%" => Ok(Units::Percent),
            "percent" => Ok(Units::Percent),
            "Percent" => Ok(Units::Percent),
            "degC" => Ok(Units::Celsius),
            "degF" => Ok(Units::Fahrenheit),
            _ => Err(format!("Invalid unit: {}", abbreviation)),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Units::Celsius => "Celsius",
            Units::Fahrenheit => "Fahrenheit",
            Units::Millimeters => "Millimeters",
            Units::Centimeters => "Centimeters",
            Units::Meters => "Meters",
            Units::Kilometers => "Kilometers",
            Units::Inches => "Inches",
            Units::Feet => "Feet",
            Units::Yards => "Yards",
            Units::Miles => "Miles",
            Units::Langley => "Langley",
            Units::MegaJoulesPerSquareMeter => "MegaJoules/Meter²",
            Units::WattsPerSquareMeter => "Watts",
            Units::Pascals => "Pascals",
            Units::KiloPascals => "KiloPascals",
            Units::Degrees => "Degrees",
            Units::Radians => "Radians",
            Units::MetersPerSecond => "Meters/Second",
            Units::MilesPerHour => "Miles/Hour",
            Units::Acres => "Acres",
            Units::Hectares => "Hectares",
            Units::SquareFeet => "Square Feet",
            Units::SquareMeters => "Square Meters",
            Units::Percent => "Percent",
        }
    }

    // convert between different units, there are several special cases to consider where meters to meters per second conversion is based on
    // total meters per day and will change to meters per second. Also miles (per day) to meters per second conversion is based on total miles per day.
    pub fn convert(&self, value: f64, to_unit: &Units) -> Result<f64, String> {
        match (self, to_unit) {
            (Units::Celsius, Units::Fahrenheit) => Ok(value * 9.0 / 5.0 + 32.0),
            (Units::Fahrenheit, Units::Celsius) => Ok((value - 32.0) * 5.0 / 9.0),
            (Units::Millimeters, Units::Centimeters) => Ok(value / 10.0),
            (Units::Centimeters, Units::Millimeters) => Ok(value * 10.0),
            (Units::Meters, Units::Kilometers) => Ok(value / 1000.0),
            (Units::Kilometers, Units::Meters) => Ok(value * 1000.0),
            (Units::Inches, Units::Feet) => Ok(value / 12.0),
            (Units::Feet, Units::Inches) => Ok(value * 12.0),
            (Units::Yards, Units::Meters) => Ok(value * 0.9144),
            (Units::Meters, Units::Yards) => Ok(value / 0.9144),
            (Units::Miles, Units::Kilometers) => Ok(value * 1.60934),
            (Units::Kilometers, Units::Miles) => Ok(value / 1.60934),
            (Units::Langley, Units::MegaJoulesPerSquareMeter) => Ok(value * 0.04184),
            (Units::MegaJoulesPerSquareMeter, Units::Langley) => Ok(value / 0.04184),
            (Units::WattsPerSquareMeter, Units::MegaJoulesPerSquareMeter) => Ok(value / 3600000.0),
            (Units::MegaJoulesPerSquareMeter, Units::WattsPerSquareMeter) => Ok(value * 3600000.0),
            (Units::KiloPascals, Units::Pascals) => Ok(value * 1000.0),
            (Units::Pascals, Units::KiloPascals) => Ok(value / 1000.0),
            (Units::Degrees, Units::Radians) => Ok(value * std::f64::consts::PI / 180.0),
            (Units::Radians, Units::Degrees) => Ok(value * 180.0 / std::f64::consts::PI),
            (Units::MetersPerSecond, Units::MilesPerHour) => Ok(value * 2.23694),
            (Units::MilesPerHour, Units::MetersPerSecond) => Ok(value / 2.23694),
            (Units::Miles, Units::MetersPerSecond) => Ok(value * 0.01864352),
            (Units::Meters, Units::MetersPerSecond) => Ok(value * 0.000011574074),  // assumes Meters per day
            (Units::Kilometers, Units::MetersPerSecond) => Ok(value * 0.0011574074),  // assumes Kilometers per day
            (Units::Acres, Units::SquareMeters) => Ok(value * 4046.86),
            (Units::SquareMeters, Units::Acres) => Ok(value / 4046.86),
            (Units::Hectares, Units::SquareMeters) => Ok(value * 10000.0),
            (Units::SquareMeters, Units::Hectares) => Ok(value / 10000.0),
            (Units::SquareFeet, Units::SquareMeters) => Ok(value / 10.7639),
            (Units::SquareMeters, Units::SquareFeet) => Ok(value * 10.7639),
            (Units::Hectares, Units::Acres) => Ok(value / 2.47105),
            (Units::Acres, Units::Hectares) => Ok(value * 2.47105),
            _ => Err(format!(
                "Unsupported conversion from {} to {}",
                self.name(),
                to_unit.name()
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_test() {
        let celsius = Units::Celsius;
        let fahrenheit = Units::Fahrenheit;
        let meters = Units::Meters;
        let kilometers = Units::Kilometers;

        assert_eq!(celsius.convert(25.0, &fahrenheit), Ok(77.0));
        assert_eq!(fahrenheit.convert(77.0, &celsius), Ok(25.0));

        assert_eq!(
            celsius.convert(25.0, &meters),
            Err(format!(
                "Unsupported conversion from {} to {}",
                celsius.name(),
                meters.name()
            ))
        );
        assert_eq!(meters.convert(25000.0, &kilometers), Ok(25.0));

        assert_eq!(
            Units::Langley.convert(1000.0, &Units::MegaJoulesPerSquareMeter),
            Ok(41.84)
        );
        assert_eq!(
            Units::MegaJoulesPerSquareMeter.convert(41.84, &Units::Langley),
            Ok(1000.0)
        );
        assert_eq!(
            Units::Langley.convert(0.0, &Units::MegaJoulesPerSquareMeter),
            Ok(0.0)
        );

        assert_eq!(
            Units::Degrees.convert(45.0, &Units::Radians),
            Ok(std::f64::consts::PI / 4.0)
        );
        assert_eq!(
            Units::Radians.convert(std::f64::consts::PI / 4.0, &Units::Degrees),
            Ok(45.0)
        );

        let mph = (Units::MetersPerSecond
            .convert(10.0, &Units::MilesPerHour)
            .unwrap()
            * 10000.0)
            .round()
            / 10000.0; // round to 4 decimal places
        assert_eq!(mph, 22.3694);

        let mps = (Units::MilesPerHour
            .convert(22.3694, &Units::MetersPerSecond)
            .unwrap()
            * 10000.0)
            .round()
            / 10000.0; // round to 4 decimal places
        assert_eq!(mps, 10.0);
    }
}
