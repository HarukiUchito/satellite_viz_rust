extern crate gnuplot;
#[macro_use]
extern crate rosrust;
#[macro_use]
extern crate rosrust_codegen;

rosmsg_include!();

use gnuplot::*;

fn main() {

    rosrust::init("satellite_viz_rust");

    let fg = std::cell::RefCell::new(Figure::new());
    
    let _subscriber_raii = rosrust::subscribe(
        "satellites",
        move |v: msg::rtklibros::satellites|
    {
        //ros_info!("Received ->");

        fg.borrow_mut().clear_axes();
        let sats = v.satellites;
        let mut azels = Vec::new();
        let mut elevs = Vec::new();
        let mut names = Vec::new();
        for i in 0..sats.len() {
            //ros_info!(
            //    "name: {}, az: {}, el: {}",
            //    sats[i].name, sats[i].azimuth, sats[i].elevation
            //);
            azels.push(sats[i].azimuth);
            elevs.push(sats[i].elevation);
            names.push(sats[i].name.clone());
        }
        
        (0..sats.len()).fold(
            fg.borrow_mut().axes2d()//.set_x_range(Fix(-50.0), Fix(50.0)).set_y_range(Fix(-50.0), Fix(50.0))
            .points(&azels, &elevs, &[Caption("G01"), PointSymbol('O'), Color("black")]),
            |ax, i| {
                ax.label(&names[i], Axis(azels[i]), Axis(elevs[i]), &[TextAlign(AlignCenter)])
            }
        );

        fg.borrow_mut().show();
    }
    ).unwrap();

    rosrust::spin();
}