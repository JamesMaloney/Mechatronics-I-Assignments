use std::{error::Error, thread, time::Duration};
use plotters::prelude::*;


#[tokio::main]
async fn main(){
    let api_key: String = "GULdU6YfSVAbL0qG3YbSMx2zRSbGsfwT".to_string();
    let mut temperatures: Vec<(f32,f32)> = vec![];
    let mut humidities: Vec<(f32, f32)> = vec![];
    let mut parsed;
    let mut counter: i32 = 0;
    loop {
        parsed = get_values(&api_key).await.unwrap();
        temperatures.push((counter.clone() as f32*5.0, parsed[0]["Temperature"]["Metric"]["Value"].clone().to_string().parse::<f32>().unwrap()));
        humidities.push((counter.clone() as f32*5.0, parsed[0]["RelativeHumidity"].clone().to_string().parse::<f32>().unwrap()));
        println!("Humidity {} and Temperature {} after {} minutes",parsed[0]["RelativeHumidity"].clone(),parsed[0]["Temperature"]["Metric"]["Value"].clone(),counter.clone()*5);
        counter+=1;
        if counter != 5 {
            thread::sleep(Duration::from_secs(5));
            continue;
        }
        break;
    }
    plot(temperatures, humidities).unwrap();
}

async fn get_values(api_key: &String) -> Result<json::JsonValue, Box<dyn Error>>{
    let mut url = "http://dataservice.accuweather.com/currentconditions/v1/190390?apikey=".to_string()+&api_key;
    url=url+&"&details=true".to_string();
    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;
    let parsed = json::parse(&body).unwrap();
    Ok(parsed)
}

fn plot(data:Vec<(f32,f32)>, second_data:Vec<(f32,f32)>) -> Result<(),Box<dyn Error>> {
    let first: Vec<(f32, f32)> = data;
    let second: Vec<(f32, f32)> = second_data;
    let root = BitMapBackend::new("graph.png", (4096, 1024)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(0)
        .y_label_area_size(0)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..20f32, 0f32..100f32)?;
    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;
    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        first.clone(),
        &RED,
    ))?;
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        first.clone(),
        30,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;
    chart.draw_series(LineSeries::new(
        second.clone(),
        &BLUE,
    ))?;
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        second.clone(),
        30,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;
    Ok(())
}