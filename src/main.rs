mod traffic_light;
mod collections_sum;
mod graph_area;

fn main() {
    let light = traffic_light::TrafficLight::Green;
    println!("traffic light time duration is {}", traffic_light::get_duration(&light));

    let list = vec![0,1,2,3,4,5];
    let sum = collections_sum::sum(&list);
    println!("sum of the array is {}", sum.unwrap());

    let list2 = vec![0,1000,2,3,4,5];
    let sum2 = collections_sum::sum(&list2);
    println!("return none, sum of the array is {}", sum2.unwrap_or(0));

    let square = graph_area::Square {
        side: 0.5
    };
    let area = graph_area::get_area(&square);
    println!("the area of the square is {}", area);

    let triangle = graph_area::Triangle {
        side: 2.0
    };
    let area2 = graph_area::get_area(&triangle);
    println!("the area of the triangle is {}", area2);
}
