# Climate Package

This is a package that will retrieve the climate data from the Postgresql database and also
includes several standardized ways to hold the data with methods that will allow
the data to be called when needed. This also includes during the loading of the data
functions to QC the climate data for common errors and other issues.

## Items Implemented

- [ ] Import from Database
- [ ] QC Temperature Data
- [ ] QC Solar Radiation Data
- [ ] QC Vapor Pressure
- [ ] QC Precipitation
- [ ] Output the struct for use in other packages

## How to use this Library

This library is intended to be the linkage between your climate data and the refet library that is used
to calculate reference ET amounts. The library has several exported structures, but the primary operations are
with the `StationData` and the `DailyData` that are used to hold the climate data for further process.

Once the data is loaded into those structures, the `StationData.to_output` is used to output the information
correctly as an input to the refet library.