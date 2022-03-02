use image::*;
use ndarray::*;
use std::thread;
use std::time::Instant;
use process::Transform;
mod process;


fn main() {
let start = Instant::now();

let path = "C:\\Users\\Antoine Demangeon\\Desktop\\perso\\code\\rust\\dinoise\\src\\image.jpg";

let image = open(path).unwrap();

let img_vec = image_to_vec(image);

let to_mat = Array::from_shape_vec((img_vec.1 as usize, img_vec.0 as usize, 3), img_vec.2).unwrap();

let rep_conv = to_mat;

//compute the derivative of the image (border of the image)
let borders = conv(&rep_conv);
let convolved_image = borders.clone();

let denoised_border_filter = thread::spawn(move ||{
//compute the derivative of the border and noise to reduce it later
let noisy_border = conv(&borders);

//we remove noise and thicken borders
(&borders*2 - noisy_border).set_range()

});

//first and second pass of denoising
let denoise_image_1 = &rep_conv - convolved_image/2;

let denoise_image_2 = &denoise_image_1 - conv(&denoise_image_1)/2;

//put the threads back together
let res_dbf = denoised_border_filter.join().unwrap();

//add border back
let final_image_mat = blur(&denoise_image_2, &res_dbf);

let final_image = array_to_image(final_image_mat);

final_image.save("out.png").unwrap();

let duration = start.elapsed();
println!("time: {:?}",duration);
}


fn array_to_image(arr: Array3<u8>) -> RgbImage {
     assert!(arr.is_standard_layout());
 
     let (height, width, _) = arr.dim();
     let raw = arr.into_raw_vec();
 
     RgbImage::from_raw(width as u32, height as u32, raw)
         .expect("container should have the right size for the image dimensions")
 }

 fn image_to_vec(img: image::DynamicImage) -> (u32, u32, Vec<u8>){

     let (width,height) = &img.to_luma8().dimensions();
     let rgb_image = img.to_rgb8();
     (*width, *height, rgb_image.into_raw())

 }


fn blur(arr: &Array3<u8>, filter: &Array3<u8>) -> Array3<u8>{

    let (height, width, _) = arr.dim();
    let mut new_array = Array3::<u8>::zeros((height, width, 3));
    for j in 2..(width-2){
        for i in 2..(height-2){
           
            for c in 0..3{

                if !cross_detection(i, j, c, filter, height, width){

                new_array[[i,j,c]]=((
                    {
                        let mut test= 0i16;
                        for x in -2..2{
                            for y in -2..2{
                                test += arr[[(i as i16+x as i16) as usize,(j as i16+y as i16) as usize,c]] as i16;

                            }
                        }

                        test

                    }
                ).max(0)/16) as u8;
                }else{
                    new_array[[i,j,c]]=arr[[i,j,c]];
                }

            }
            
        }
    }

    new_array


}

fn cross_detection(i: usize, j: usize,c: usize, filter : &Array3<u8>, height: usize, width: usize) -> bool{

        let mut cross = 0; 
        let range: isize = 4;

        for xaxis in -range..range{

            for yaxis in -range..range{ 

                if xaxis+(i as isize) > 0 && xaxis+(i as isize) < height as isize{
                if yaxis+(j as isize) > 0 && yaxis+(j as isize) < width as isize{
                    cross+=filter[[(xaxis+(i as isize)) as usize,(yaxis+(j as isize)) as usize,c]];
                    
                }}
    
            }


        }


        let res = cross>=1;

        return res;

}

fn conv(arr: &Array3<u8>) -> Array3<u8> {

    let (height, width, _) = arr.dim();
    let mut new_array = Array3::<u8>::zeros((height, width, 3));
    for j in 0..(width-1){
        for i in 0..(height-1){
           
            for c in 0..3{

                new_array[[i,j,c]]=(
                 (arr[[i,j,c]] as i16)*2
                -(arr[[i+1,j,c]] as i16)
                -(arr[[i,j+1,c]] as i16))
                .max(0) as u8;

            }
            
        }
    }

    

    new_array

}

