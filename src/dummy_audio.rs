use projectm_rs::core::{ProjectMHandle, Projectm};

#[allow(dead_code)]
pub fn generate_random_audio_data(projectm_handle: ProjectMHandle) {
    let mut pcm_data: [[libc::c_short; 512]; 2] = [[0; 512]; 2];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        if i % 2 as libc::c_int == 1 as libc::c_int {
            pcm_data[0 as libc::c_int as usize][i as usize] =
                -(pcm_data[0 as libc::c_int as usize][i as usize] as libc::c_int) as libc::c_short;
            pcm_data[1 as libc::c_int as usize][i as usize] =
                -(pcm_data[1 as libc::c_int as usize][i as usize] as libc::c_int) as libc::c_short
        }
        i += 1
    }

    Projectm::pcm_add_int16(projectm_handle, vec![pcm_data[0][0]], 2)
}
