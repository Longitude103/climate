use chrono::{DateTime, Utc};

pub struct Output {
    tmax: f64,  // temperature maximum in Celsius
    tmin: f64,  // temperature minimum in Celsius
    rhmax: Option<f64>,  // relative humidity maximum in %
    rhmin: Option<f64>,  // relative humidity minimum in %
    dewpoint: Option<f64>,  // dewpoint temperature in Celsius
    ea: Option<f64>,  // vapor pressure deficit in kPa
    rs: Option<f64>,  // solar radiation in MJ/m²
    ws: Option<f64>,  // wind speed in m/s
    wz: Option<f64>,  // height of wind speed m
    z: f64,  // elevation in m
    latitude: f64,  // latitude in radians
    date: DateTime<Utc>, // date and time of the reading
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

impl Output {
    pub fn new_with_values(tmax: f64, tmin: f64, rhmax: Option<f64>, rhmin: Option<f64>, dewpoint: Option<f64>, ea: Option<f64>,
                           rs: Option<f64>, ws: Option<f64>, wz: Option<f64>, z: f64, latitude: f64, date: DateTime<Utc>) -> Output {
        Output {
            tmax,
            tmin,
            rhmax,
            rhmin,
            dewpoint,
            ea,
            rs,
            ws,
            wz,
            z,
            latitude,
            date,
        }
    }

    pub fn new() -> Output {
        Output {
            tmax: 0.0,
            tmin: 0.0,
            rhmax: None,
            rhmin: None,
            dewpoint: None,
            ea: None,
            rs: None,
            ws: None,
            wz: None,
            z: 0.0,
            latitude: 0.0,
            date: Utc::now(),
        }
    }

    // create getters for each field
    // tmax getters and setters
    pub fn get_tmax(&self) -> f64 {
        self.tmax
    }

    pub fn set_tmax(&mut self, tmax: f64) {
        self.tmax = tmax;
    }

    // tmin getters and setters
    pub fn get_tmin(&self) -> f64 {
        self.tmin
    }

    pub fn set_tmin(&mut self, tmin: f64) {
        self.tmin = tmin;
    }

    // rhmax getters and setters
    pub fn get_rhmax(&self) -> Option<f64> {
        self.rhmax
    }

    pub fn set_rhmax(&mut self, rhmax: Option<f64>) {
        self.rhmax = rhmax;
    }

    // rhmin getters and setters
    pub fn get_rhmin(&self) -> Option<f64> {
        self.rhmin
    }

    pub fn set_rhmin(&mut self, rhmin: Option<f64>) {
        self.rhmin = rhmin;
    }

    // dewpoint getters and setters
    pub fn get_dewpoint(&self) -> Option<f64> {
        self.dewpoint
    }

    pub fn set_dewpoint(&mut self, dewpoint: Option<f64>) {
        self.dewpoint = dewpoint;
    }

    // ea getters and setters
    pub fn get_ea(&self) -> Option<f64> {
        self.ea
    }

    pub fn set_ea(&mut self, ea: Option<f64>) {
        self.ea = ea;
    }

    // rs getters and setters
    pub fn get_rs(&self) -> Option<f64> {
        self.rs
    }

    pub fn set_rs(&mut self, rs: Option<f64>) {
        self.rs = rs;
    }

    // ws getters and setters
    pub fn get_ws(&self) -> Option<f64> {
        self.ws
    }

    pub fn set_ws(&mut self, ws: Option<f64>) {
        self.ws = ws;
    }

    // wz getters and setters
    pub fn get_wz(&self) -> f64 {
        if self.wz.is_some() {
            self.wz.unwrap()
        } else {
            2.0  // default to 2.0 meters is the standard height for these stations
        }
    }

    pub fn set_wz(&mut self, wz: Option<f64>) {
        self.wz = wz;
    }

    // z getters and setters
    pub fn get_z(&self) -> f64 {
        self.z
    }

    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }

    // latitude getters and setters
    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    // set_latitude changes the latitude of the station from degrees to radians
    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude.to_radians();
    }

    // date getters and setters
    pub fn get_date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn set_date(&mut self, date: DateTime<Utc>) {
        self.date = date;
    }
}