use ndarray::{Array3};


pub trait Transform{

    fn set_range(&self) -> ndarray::Array3<u8>;

}

impl Transform for ndarray::Array3<u8>{


    fn set_range(&self) -> ndarray::Array3<u8> {

        let mut com_array = Array3::<u8>::zeros(self.dim());
        let (height, width, _) = self.dim();

        for y in 0..width{
            for x in 0..height{

                if (self[[x,y,0]] as u16) + (self[[x,y,1]] as u16) + (self[[x,y,2]] as u16) > 260 {

                    com_array[[x,y,0]]=1;
                    com_array[[x,y,1]]=1;
                    com_array[[x,y,2]]=1;

                }

            }
        }


        return com_array;

     }

}


