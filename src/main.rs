use anyhow::Result;
use opencv::{
    prelude::*,
    videoio,
    objdetect,
    imgproc,
    highgui,
    types::VectorOfRect,
    core::Size, core::Scalar, core::Rect
};

fn center_of(rect: &Rect) -> (i32, i32) {
    let center_x = rect.x + (rect.width / 2);
    let center_y = rect.y + (rect.height / 2);
    (center_x, center_y)
}

/// Calculate the square of the distance from the center of a frame for a rectangle
/// No need to take the square root - just to choose the smallest
fn distance_squared(center: &Size, rect: &Rect) -> i64 {
    let (center_x, center_y) = center_of(rect);
    let distance_x = center_x - (center.width / 2);
    let distance_y = center_y - (center.height / 2);
    ((distance_x * distance_x) + (distance_y * distance_y)) as i64
}

fn most_centered<'a>(frame: Size, candidates: VectorOfRect) -> Rect {
    let mut ordered = candidates.iter().map(|c| (distance_squared(&frame, &c), c)).collect::<Vec<(i64, Rect)>>();
    ordered.sort_by(|(d1, _), (d2, _)| d1.cmp(d2));

    ordered.get(0).unwrap().1
}

fn main() -> Result<()> { // Note, this is anyhow::Result
    // Open a GUI window
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    // Open the web-camera (assuming you have one)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default(); // This array will store the webcam data

    // Chose a model from:
    // haarcascade_frontalface_alt.xml
    // haarcascade_frontalface_alt2.xml
    // haarcascade_frontalface_alt_tree.xml
    // haarcascade_frontalface_default.xml
    let xml = "/opt/homebrew/Cellar/opencv/4.10.0_4/share/opencv4/haarcascades/haarcascade_frontalface_alt_tree.xml";
    let mut detector = objdetect::CascadeClassifier::new(xml)?;

    // get an initial frame to calculate the center location
    cam.read(&mut frame)?;
    let size = frame.size()?;
    let nominal_location = (0.5, 0.5);
    let nominal_size = (390, 390);
    let mut location = (0.5, 0.5);

    loop {
        // Read the camera and display in the window
        cam.read(&mut frame)?;

        // convert to greyscale
        let mut grey = Mat::default();
        imgproc::cvt_color(&frame, &mut grey, imgproc::COLOR_BGR2GRAY, 0)?;

        // create a vec of rectangles for faces
        let mut faces = VectorOfRect::new();

        // detect faces in the image
        detector.detect_multi_scale(&grey, &mut faces, 1.1, 2, objdetect::CASCADE_SCALE_IMAGE,
        Size::new(100, 100), Size::new(0, 0));

        if !faces.is_empty() {
            // find the most centered face
            let main_face =  most_centered(frame.size()?, faces);
            let center = center_of(&main_face);
            location = (center.0 as f32 /size.width as f32, center.1 as f32 /size.height as f32);

            imgproc::rectangle(
                &mut frame,
                main_face,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_8,
                0
            )?;

            println!("Size difference: {:?}", (nominal_size.0 - main_face.width, nominal_size.1 - main_face.height));
        }

        let displacement = (location.0 - nominal_location.0, location.1 - nominal_location.1);
        println!("Displacement: {:?}", displacement);
        highgui::imshow("window", &frame)?;
    }
    Ok(())
}