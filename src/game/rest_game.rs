//cuando la mod name {} tiene un cuerpo le indica a rust que el cuerpo podrá ser accedido mediante use crate::

pub mod hosting {
    pub fn test_rest() {

        // declarando un vector
        let mut v = vec![1,2,3];

        //  asignandole un nuevo valor al vector
        v = vec![4, 5, 6];

        // añadiendo un nuevo elemtno al vector
        v.push(4);

        println!("holas desde test_rest {}", &v[2]);

        // iterando un vector

        for element in &v {
            println!("Elemento del vector {}", element);
        }
    }
}
