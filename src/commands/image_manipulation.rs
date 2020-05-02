use image;

use std::fs::File;
use std::io::Write;
use std::borrow::Cow;
use std::sync::{
    Arc,
    Mutex,
};

use serenity::{
    prelude::Context,
    model::channel::Message,
    http::AttachmentType,
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
};
use tokio::task::block_in_place;

async fn gay(image_vec: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>{
    // Load the image as a buffer.
    let imgbuf = image::load_from_memory(&image_vec)?
        .into_rgba();

    let x = imgbuf.width();
    let y = imgbuf.height();
    let (mut pos_x, mut pos_y, mut iteration) = (0, 0, 0);

    let mut imgbuf_clone = imgbuf.clone();
    let mut gay_bytes = Vec::new();

    //imgbuf = image::imageops::resize(&imgbuf, 100, 100, image::imageops::FilterType::Triangle);
    while x > pos_x + 20 + iteration && y > pos_y + 20 + iteration {
        iteration += 5;
        pos_x += 20 + iteration;
        pos_y += 20 + iteration;

        image::imageops::overlay(&mut imgbuf_clone, &imgbuf, pos_x, pos_y);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    // imgbuf.save("grayscale.png")?;
    image::DynamicImage::ImageRgba8(imgbuf_clone)
        .write_to(&mut gay_bytes, image::ImageOutputFormat::Jpeg(255))
        .expect("There was an error writing the image.");

    Ok(gay_bytes.to_vec())
    //Ok(Vec::new())
}

async fn grayscale(image_vec: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>{
    // Load the image as a buffer.
    let mut imgbuf = image::load_from_memory(&image_vec)?
        .into_rgba();

    let gray_bytes = Arc::new(Mutex::new(Vec::new()));
    let gray_bytes_clone = Arc::clone(&gray_bytes);

    // Iterate over the coordinates and pixels of the image
    // This makes the grading.
    block_in_place(move || {
        for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
            // Algorythm to transform RGB into black and white.
            // https://en.wikipedia.org/wiki/YIQ
            let r = (pixel.0[0] as f32 * 0.299 as f32).abs() as u8;
            let g = (pixel.0[1] as f32 * 0.587 as f32).abs() as u8;
            let b = (pixel.0[2] as f32 * 0.114 as f32).abs() as u8;

            let gray = r+g+b;

            *pixel = image::Rgba([gray, gray, gray, pixel.0[3]]);
        }

        // Save the image as “fractal.png”, the format is deduced from the path
        // imgbuf.save("grayscale.png")?;
        image::DynamicImage::ImageRgba8(imgbuf)
            .write_to(&mut *gray_bytes_clone.lock().unwrap(), image::ImageOutputFormat::Jpeg(255))
            .expect("There was an error writing the image.");
    });

    let result = gray_bytes.lock().unwrap();
    Ok(result.clone().to_vec())
}

#[command]
#[aliases(grayscale)]
async fn gray(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    // obtains the first attachment on the message or None if the message doesn't have one.
    let first_attachment = &msg.attachments.get(0);
    let mut filename = &String::new();

    let (image_url, bytes) = match first_attachment {
        // if there was an attachment on the first possition, unwrap it.
        Some(x) => {
            // get the dimensions of the image.
            let dimensions = x.dimensions();

            // if the dimensions is None, it means it's not an image, but a normal file, so we respond acordingly.
            if dimensions == None { 
                let err_message = "The provided file is not a valid image.".to_string();
                (err_message, vec![0])
            // else we download the image. Download returns a Result Vec<u8>
            } else {
                if dimensions.unwrap().0 > 7680 || dimensions.unwrap().1 >  4320 {
                    msg.reply(ctx, "The provided image is too large").await?;
                    return Ok(());
                }

                let bytes = x.download().await?;
                filename = &x.filename;

                let mut file = File::create(filename)?;
                file.write_all(&bytes)?;

                (x.url.to_owned(), bytes)
            }
        },
        // else say that an image was not provided.
        None => ("No image was provided.".to_string(), vec![0])
    };

    // if an error was returned from the previous checks, say the error and finish the command.
    if bytes == vec![0] {
        msg.channel_id.say(ctx, image_url).await?;
        return Ok(());
    }

    // Uploads the grayscaled image bytes as an attachment
    // this is necessary to do as im never saving the image, just have the bytes as a vector.
    let grayscaled_bytes = grayscale(&bytes).await?;
    let attachment = AttachmentType::Bytes {
        data: Cow::from(grayscaled_bytes),
        filename: filename.to_owned(),
    };

    // Sends an embed with a link to the original image ~~and the prided image attached~~.
    msg.channel_id.send_message(ctx, |m| {
        m.add_file(attachment);
        m.embed(|e| {
            e.title("Original Image");
            e.url(image_url);
            e.image(format!("attachment://{}", filename));
            e
        });
        m
    }).await?;

    Ok(())
}
#[command]
#[aliases(gay)]
async fn pride(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let first_attachment = &msg.attachments.get(0);
    let mut filename = &String::new();

    let (image_url, bytes) = match first_attachment {
        // if there was an attachment on the first possition, unwrap it.
        Some(x) => {
            // get the dimensions of the image.
            let dimensions = x.dimensions();

            // if the dimensions is None, it means it's not an image, but a normal file, so we respond acordingly.
            if dimensions == None { 
                let err_message = "The provided file is not a valid image.".to_string();
                (err_message, vec![0])
            // else we download the image. Download returns a Result Vec<u8>
            } else {
                if dimensions.unwrap().0 > 7680 || dimensions.unwrap().1 >  4320 {
                    msg.reply(ctx, "The provided image is too large").await?;
                    return Ok(());
                }

                let bytes = x.download().await?;
                filename = &x.filename;

                let mut file = File::create(filename)?;
                file.write_all(&bytes)?;

                (x.url.to_owned(), bytes)
            }
        },
        // else say that an image was not provided.
        None => ("No image was provided.".to_string(), vec![0])
    };

    // if an error was returned from the previous checks, say the error and finish the command.
    if bytes == vec![0] {
        msg.channel_id.say(ctx, image_url).await?;
        return Ok(());
    }

    // Uploads the grayscaled image bytes as an attachment
    // this is necessary to do as im never saving the image, just have the bytes as a vector.
    let grayscaled_bytes = gay(&bytes).await?;
    let attachment = AttachmentType::Bytes {
        data: Cow::from(grayscaled_bytes),
        filename: filename.to_owned(),
    };

    // Sends an embed with a link to the original image ~~and the prided image attached~~.
    msg.channel_id.send_message(ctx, |m| {
        m.add_file(attachment);
        m.embed(|e| {
            e.title("Original Image");
            e.url(image_url);
            e.image(format!("attachment://{}", filename));
            e
        });
        m
    }).await?;

    Ok(())
}
