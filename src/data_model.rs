use crate::output::Output;
use crate::units::Units;
use chrono::NaiveDate;
use std::error::Error;

pub struct DailyData {
    pub date: NaiveDate,
    pub tmin: f64,
    pub tmin_units: String,
    pub tmax: f64,
    pub tmax_units: String,
    pub rhmin: Option<f64>,
    pub rhmin_units: Option<String>,
    pub rhmax: Option<f64>,
    pub rhmax_units: Option<String>,
    pub dewpoint: Option<f64>,
    pub dewpoint_units: Option<String>,
    pub precip: Option<f64>,
    pub precip_units: Option<String>,
    pub rs: Option<f64>,
    pub rs_units: Option<String>,
    pub ea: Option<f64>,
    pub ea_units: Option<String>,
    pub wind_speed: Option<f64>,
    pub ws_units: Option<String>,
    pub short_refet: Option<f64>,
    pub tall_refet: Option<f64>,
}

impl DailyData {
    /// Creates a new instance of `DailyData` with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `date` - A string representing the date of the data.
    /// * `tmin` - A tuple containing the minimum temperature value and its units.
    /// * `tmax` - A tuple containing the maximum temperature value and its units.
    /// * `rhmin` - A tuple containing the minimum relative humidity value and its units.
    /// * `rhmax` - A tuple containing the maximum relative humidity value and its units.
    /// * `dewpoint` - A tuple containing the dewpoint value and its units.
    /// * `precip` - A tuple containing the precipitation value and its units.
    /// * `rs` - A tuple containing the solar radiation value and its units.
    /// * `ea` - A tuple containing the vapor pressure value and its units.
    /// * `wind_speed` - A tuple containing the wind speed value and its units.
    ///
    /// # Returns
    ///
    /// A new instance of `DailyData`.
    pub fn new(
        date: NaiveDate,
        tmin: (f64, String),
        tmax: (f64, String),
        rhmin: Option<(f64, String)>,
        rhmax: Option<(f64, String)>,
        dewpoint: Option<(f64, String)>,
        precip: Option<(f64, String)>,
        rs: Option<(f64, String)>,
        ea: Option<(f64, String)>,
        wind_speed: Option<(f64, String)>,
    ) -> Result<DailyData, String> {
        let (tmin_value, tmin_units) = tmin;
        let (tmax_value, tmax_units) = tmax;

        let mut daily_data = DailyData {
            date,
            tmin: tmin_value,
            tmin_units,
            tmax: tmax_value,
            tmax_units,
            rhmin: None,
            rhmin_units: None,
            rhmax: None,
            rhmax_units: None,
            dewpoint: None,
            dewpoint_units: None,
            precip: None,
            precip_units: None,
            rs: None,
            rs_units: None,
            ea: None,
            ea_units: None,
            wind_speed: None,
            ws_units: None,
            short_refet: None,
            tall_refet: None,
        };

        if let Some((rhmin_value, rhmin_units)) = rhmin {
            daily_data.rhmin = Some(rhmin_value);
            if rhmin_units.is_empty() {
                return Err(
                    "Relative humidity min units must not be empty when including a value"
                        .to_string(),
                );
            }

            daily_data.rhmin_units = Some(rhmin_units);
        }

        if let Some((rhmax_value, rhmax_units)) = rhmax {
            daily_data.rhmax = Some(rhmax_value);
            if rhmax_units.is_empty() {
                return Err(
                    "Relative humidity max units must not be empty when including a value"
                        .to_string(),
                );
            }

            daily_data.rhmax_units = Some(rhmax_units);
        }
        if let Some((dewpoint_value, dewpoint_units)) = dewpoint {
            daily_data.dewpoint = Some(dewpoint_value);
            if dewpoint_units.is_empty() {
                return Err("Dewpoint units must not be empty when including a value".to_string());
            }

            daily_data.dewpoint_units = Some(dewpoint_units);
        }
        if let Some((precip_value, precip_units)) = precip {
            daily_data.precip = Some(precip_value);
            if precip_units.is_empty() {
                return Err(
                    "Precipitation units must not be empty when including a value".to_string(),
                );
            }

            daily_data.precip_units = Some(precip_units);
        }
        if let Some((rs_value, rs_units)) = rs {
            daily_data.rs = Some(rs_value);
            if rs_units.is_empty() {
                return Err(
                    "Solar radiation units must not be empty when including a value".to_string(),
                );
            }

            daily_data.rs_units = Some(rs_units);
        }
        if let Some((ea_value, ea_units)) = ea {
            daily_data.ea = Some(ea_value);
            if ea_units.is_empty() {
                return Err(
                    "Vapor pressure units must not be empty when including a value".to_string(),
                );
            }

            daily_data.ea_units = Some(ea_units);
        }
        if let Some((ws_value, ws_units)) = wind_speed {
            daily_data.wind_speed = Some(ws_value);
            if ws_units.is_empty() {
                return Err("Wind speed units must not be empty when including a value".to_string());
            }
            daily_data.ws_units = Some(ws_units);
        }

        Ok(daily_data)
    }

    fn to_output(&self) -> Result<Output, String> {
        let mut output = Output::new();
        output.set_date(self.date.clone());
        output.set_tmin(convert_temp_to_c(self.tmin, &*self.tmin_units)?);
        output.set_tmax(convert_temp_to_c(self.tmax, &*self.tmax_units)?);
        if self.dewpoint.is_some() {
            let d_units = self.dewpoint_units.as_ref().unwrap().clone();
            let dewpoint_converted = convert_temp_to_c(self.dewpoint.unwrap(), &d_units)?;
            output.set_dewpoint(Some(dewpoint_converted));
        }

        if self.rhmin.is_some() {
            let r_units = self.rhmin_units.as_ref().unwrap().clone();
            let r_unit =
                Units::from_abbreviation(&r_units).expect("Invalid units for relative humidity");
            match r_unit {
                Units::Percent => {
                    output.set_rhmin(Some(self.rhmin.unwrap()));
                }
                _ => {
                    return Err(
                        "Invalid units for relative humidity min, must be percent".to_string()
                    )
                }
            }
        }

        if self.rhmax.is_some() {
            let r_units = self.rhmax_units.as_ref().unwrap().clone();
            let r_unit =
                Units::from_abbreviation(&r_units).expect("Invalid units for relative humidity");
            match r_unit {
                Units::Percent => {
                    output.set_rhmax(Some(self.rhmin.unwrap()));
                }
                _ => {
                    return Err(
                        "Invalid units for relative humidity max, must be percent".to_string()
                    )
                }
            }
        }

        if self.ea.is_some() {
            let ea_units = self.ea_units.as_ref().unwrap().clone();
            let ea_unit =
                Units::from_abbreviation(&ea_units).expect("Invalid units for vapor pressure");
            match ea_unit {
                Units::KiloPascals => output.set_ea(Some(self.ea.unwrap())),
                Units::Pascals => output.set_ea(Some(
                    Units::Pascals
                        .convert(self.ea.unwrap(), &Units::KiloPascals)
                        .expect("Units conversion failed"),
                )),
                _ => panic!("Invalid units for ea: {}", ea_units),
            }
        }

        if self.rs.is_some() {
            let rs_units = self.rs_units.as_ref().unwrap().clone();
            let rs_unit =
                Units::from_abbreviation(&rs_units).expect("Invalid units for solar radiation");
            match rs_unit {
                Units::MegaJoulesPerSquareMeter => output.set_rs(Some(self.rs.unwrap())),
                Units::WattsPerSquareMeter => output.set_rs(Some(
                    Units::WattsPerSquareMeter
                        .convert(self.rs.unwrap(), &Units::MegaJoulesPerSquareMeter)
                        .expect("Units conversion failed"),
                )),
                Units::Langley => output.set_rs(Some(
                    Units::Langley
                        .convert(self.rs.unwrap(), &Units::MegaJoulesPerSquareMeter)
                        .expect("Units conversion failed"),
                )),
                _ => panic!("Invalid units for rs: {}", rs_units),
            }
        }

        if self.wind_speed.is_some() {
            let ws_units = self.ws_units.as_ref().unwrap().clone();
            let ws_unit =
                Units::from_abbreviation(&ws_units).expect("Invalid units for wind speed");
            match ws_unit {
                Units::MetersPerSecond => output.set_ws(Some(self.wind_speed.unwrap())),
                Units::MilesPerHour => output.set_ws(Some(
                    Units::MilesPerHour
                        .convert(self.wind_speed.unwrap(), &Units::MetersPerSecond)
                        .expect("Units conversion failed"),
                )),
                Units::Miles => output.set_ws(Some(
                    Units::Miles
                        .convert(self.wind_speed.unwrap(), &Units::MetersPerSecond)
                        .expect("Units conversion failed"),
                )),
                Units::Meters => output.set_ws(Some(
                    Units::Meters
                        .convert(self.wind_speed.unwrap(), &Units::MetersPerSecond)
                        .expect("Units conversion failed"),
                )),
                Units::Kilometers => output.set_ws(Some(
                    Units::Kilometers
                        .convert(self.wind_speed.unwrap(), &Units::MetersPerSecond)
                        .expect("Units conversion failed"),
                )),
                _ => panic!("Invalid units for ws: {}", ws_units),
            }
        }

        Ok(output)
    }
}

fn convert_temp_to_c(value: f64, actual_units: &str) -> Result<f64, String> {
    let tmin_unit = Units::from_abbreviation(actual_units)?;
    match tmin_unit {
        Units::Celsius => Ok(value),
        Units::Fahrenheit => Ok(Units::Fahrenheit.convert(value, &Units::Celsius)?),
        _ => Err(format!("Invalid units for temperature: {}", actual_units)),
    }
}

pub struct StationData {
    pub name: String,
    pub source: String,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub wind_height: f64,
    pub daily_data: Vec<DailyData>,
    pub id: Option<i32>,
}

impl StationData {
    pub fn new(
        name: String,
        source: String,
        latitude: f64,
        longitude: f64,
        elevation: f64,
        wind_height: f64,
        id: Option<i32>,
    ) -> StationData {
        StationData {
            name,
            source,
            latitude,
            longitude,
            elevation,
            wind_height,
            daily_data: Vec::new(),
            id,
        }
    }

    pub fn add_daily_data(
        &mut self,
        date: NaiveDate,
        tmin: (f64, String),            // (value, units)
        tmax: (f64, String),            // (value, units)
        rhmin: (f64, String),           // (value, units)
        rhmax: (f64, String),           // (value, units)
        dewpoint: (f64, String),        // (value, units)
        precip: (f64, String),          // (value, units)
        radiation_solar: (f64, String), // (value, units)
        ea: (f64, String),              // (value, units)
        wind_speed: (f64, String),      // (value, units)
    ) -> Result<(), Box<dyn Error>> {
        let daily_data = DailyData::new(
            date,
            tmin,
            tmax,
            Option::from(rhmin),
            Option::from(rhmax),
            Option::from(dewpoint),
            Option::from(precip),
            Option::from(radiation_solar),
            Option::from(ea),
            Option::from(wind_speed),
        );
        self.daily_data.push(daily_data?);
        Ok(())
    }

    pub fn add_daily_records(&mut self, records: Vec<DailyData>) {
        self.daily_data = records;
    }

    pub fn to_output(&self) -> Result<Vec<Output>, String> {
        let mut result: Vec<Output> = Vec::new();

        for daily_data in &self.daily_data {
            let mut output = daily_data.to_output()?;

            output.set_latitude(self.latitude);
            output.set_z(self.elevation);
            output.set_wz(Some(self.wind_height));

            result.push(output);
        }

        Ok(result)
    }
}
