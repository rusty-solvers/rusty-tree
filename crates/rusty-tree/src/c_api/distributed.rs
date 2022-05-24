//! Wrappers for Distributed Tree interface
use mpi::{ffi::MPI_Comm, topology::UserCommunicator, traits::*};
use std::ffi::CString;
use std::os::raw::c_char;

use crate::{
    data::{HDF5, JSON, VTK},
    distributed::DistributedTree,
    types::{
        domain::Domain,
        morton::MortonKey,
        point::{Point, PointType},
    },
};

#[no_mangle]
pub extern "C" fn distributed_tree_from_points(
    p_points: *const [PointType; 3],
    npoints: usize,
    balanced: bool,
    world: *mut usize,
) -> *mut DistributedTree {
    let points = unsafe { std::slice::from_raw_parts(p_points, npoints) };
    let world = std::mem::ManuallyDrop::new(unsafe {
        UserCommunicator::from_raw(*(world as *const MPI_Comm)).unwrap()
    });
    Box::into_raw(Box::new(DistributedTree::new(points, balanced, &world)))
}

#[no_mangle]
pub extern "C" fn distributed_tree_nkeys(p_tree: *const DistributedTree) -> usize {
    let tree = unsafe { &*p_tree };
    tree.keys.len()
}

#[no_mangle]
pub extern "C" fn distributed_tree_npoints(p_tree: *const DistributedTree) -> usize {
    let tree = unsafe { &*p_tree };
    tree.points.len()
}

#[no_mangle]
pub extern "C" fn distributed_tree_keys(p_tree: *const DistributedTree) -> *const MortonKey {
    let tree = unsafe { &*p_tree };
    tree.keys.as_ptr()
}

#[no_mangle]
pub extern "C" fn distributed_tree_points(p_tree: *const DistributedTree) -> *const Point {
    let tree = unsafe { &*p_tree };
    tree.points.as_ptr()
}

#[no_mangle]
pub extern "C" fn distributed_tree_balanced(p_tree: *const DistributedTree) -> bool {
    let tree = unsafe { &*p_tree };
    tree.balanced
}

#[no_mangle]
pub extern "C" fn distributed_tree_write_vtk(
    comm: *mut usize,
    p_tree: *const DistributedTree,
    p_filename: *mut c_char,
) {
    let filename = unsafe { CString::from_raw(p_filename).to_str().unwrap().to_string() };
    let tree = unsafe { &*p_tree };
    let raw_points: Vec<[PointType; 3]> = tree.points.iter().map(|p| p.coordinate).collect();

    let comm = std::mem::ManuallyDrop::new(unsafe {
        UserCommunicator::from_raw(*(comm as *const MPI_Comm)).unwrap()
    });

    let domain = Domain::from_global_points(&raw_points[..], &comm);
    tree.keys.write_vtk(filename, &domain);
}

#[no_mangle]
pub extern "C" fn distributed_tree_write_hdf5(
    comm: *mut usize,
    p_tree: *const DistributedTree,
    p_filename: *mut c_char,
) {
    let filename = unsafe { CString::from_raw(p_filename).to_str().unwrap().to_string() };
    let tree = unsafe { &*p_tree };

    let comm = std::mem::ManuallyDrop::new(unsafe {
        UserCommunicator::from_raw(*(comm as *const MPI_Comm)).unwrap()
    });

    DistributedTree::write_hdf5(&comm, filename, tree);
}

#[no_mangle]
pub extern "C" fn distributed_tree_read_hdf5(
    world: *mut usize,
    p_filepath: *mut c_char,
) -> *mut DistributedTree {
    let filepath = unsafe { CString::from_raw(p_filepath).to_str().unwrap().to_string() };
    let world = std::mem::ManuallyDrop::new(unsafe {
        UserCommunicator::from_raw(*(world as *const MPI_Comm)).unwrap()
    });

    Box::into_raw(Box::new(DistributedTree::read_hdf5(&world, filepath)))
}
