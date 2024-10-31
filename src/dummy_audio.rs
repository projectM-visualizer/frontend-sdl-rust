use crate::app::ProjectMWrapped;

#[allow(dead_code)]
pub fn generate_random_audio_data(pm: &ProjectMWrapped) {
    // Create a Vec<i16> with 1024 elements
    // two channels of 512 samples each
    let mut pcm_data: Vec<i16> = vec![0; 1024];

    for i in 0..512 {
        if i % 2 == 1 {
            pcm_data[i * 2] = -(pcm_data[i * 2] as i32) as i16;
            pcm_data[i * 2 + 1] = -(pcm_data[i * 2 + 1] as i32) as i16;
        }
    }

    pm.pcm_add_int16(pcm_data, 2);
}
