
extern crate rand;
extern crate crypto;

use crypto::{symmetriccipher, buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};


use std::str;


// Encrypt a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
fn encrypt(data: &[u8],
           key: &[u8],
           iv: &[u8])
           -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor = aes::cbc_encryptor(aes::KeySize::KeySize256,
                                           key,
                                           iv,
                                           blockmodes::PkcsPadding);

    // Each encryption operation encrypts some data from
    // an input buffer into an output buffer. Those buffers
    // must be instances of RefReaderBuffer and RefWriteBuffer
    // (respectively) which keep track of how much data has been
    // read from or written to them.
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    // Each encryption operation will "make progress". "Making progress"
    // is a bit loosely defined, but basically, at the end of each operation
    // either BufferUnderflow or BufferOverflow will be returned (unless
    // there was an error). If the return value is BufferUnderflow, it means
    // that the operation ended while wanting more input data. If the return
    // value is BufferOverflow, it means that the operation ended because it
    // needed more space to output data. As long as the next call to the encryption
    // operation provides the space that was requested (either more input data
    // or more output space), the operation is guaranteed to get closer to
    // completing the full operation - ie: "make progress".
    //
    // Here, we pass the data to encrypt to the enryptor along with a fixed-size
    // output buffer. The 'true' flag indicates that the end of the data that
    // is to be encrypted is included in the input buffer (which is true, since
    // the input data includes all the data to encrypt). After each call, we copy
    // any output data to our result Vec. If we get a BufferOverflow, we keep
    // going in the loop since it means that there is more work to do. We can
    // complete as soon as we get a BufferUnderflow since the encryptor is telling
    // us that it stopped processing data due to not having any more data in the
    // input buffer.
    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}
// Decrypts a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
//
// This function is very similar to encrypt(), so, please reference
// comments in that function. In non-example code, if desired, it is possible to
// share much of the implementation using closures to hide the operation
// being performed. However, such code would make this example less clear.
fn decrypt(encrypted_data: &[u8],
           key: &[u8],
           iv: &[u8])
           -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(aes::KeySize::KeySize256,
                                           key,
                                           iv,
                                           blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

fn main() {
    let message = r#"|Where|Name|Password|
|---|---|---|
|Evernote|yinxiangpsycho@qq.com|640916aa|
|Github|626377748@qq.com|640916aa|
|Opera|626377748@qq.com|640916|
|QQ|626377748|640916aa|
||1602030365|pickup123|
||240802111|pickup240802|
|Safari Online|asdsadsadas1|pickup123|
|阿里云|1602030365@qq.com|640916aa|
|百度网盘|psycho-a|640916a|
||psychochou|640916a|
||psychochou@163.com|psychochou123|
|必应|psychochouxz@hotmail.com|640916aa|
|京东|13412932645|640916a|
|手机|18312547401||
||13412932645||
|淘宝|psycho_euphoria|640917a|
||13422954076|640916a|
|知乎|QQ 1445747555|640916aa|
|Ubuntu||640916|"#;

    let key: [u8; 32] = [108, 195, 6, 208, 41, 196, 239, 205, 83, 4, 190, 37, 28, 15, 208, 248,
                         227, 16, 166, 78, 228, 153, 241, 41, 36, 236, 168, 29, 199, 178, 126, 92];
    let iv: [u8; 16] = [218, 132, 27, 69, 146, 121, 1, 217, 113, 203, 109, 147, 92, 31, 215, 202];

  //  let encrypted = [37, 228, 107, 54, 145, 252, 179, 80, 66, 245, 202, 185, 219, 91, 250, 186];

    // &encrypted_data[..]
      let encrypted_data = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
     println!("{:?}", encrypted_data);

    //let decrypted_data = decrypt(&encrypted, &key, &iv).ok().unwrap();
//    println!("{:?}", str::from_utf8(&decrypted_data));
    // assert!(message.as_bytes() == &decrypted_data[..]);
}
