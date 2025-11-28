use tokio::{pin, spawn, sync::mpsc};


async fn acttion(input: Option<i32>) -> Option<String> {
   
   let i = input?;

   let s = i.to_string();
    
    Some(s) 

}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::channel(100);

    let mut done = false;
    let operation = acttion(None);
    pin!(operation);

    let handle = spawn(async move {
        tx.send(1).await.unwrap();
        tx.send(3).await.unwrap();
        tx.send(2).await.unwrap();  
    }); 
        

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                println!("operation done"); 
                done = true;
                if let Some(input) = res {
                    println!("GOT = {}", input);
                    return;
                }
                   
                 
            },
            Some(input) = rx.recv() => {
                println!("received {}", input);
                if input % 2 == 0 {
                    operation.set(acttion(Some(input)));
                    done = false;   
                }
            }, 
        }
    }

    
}
