use lzxd::{Lzxd, WindowSize};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass]
struct LzxdDecoder {
    inner: Lzxd,
}

#[pymethods]
impl LzxdDecoder {
    #[new]
    fn new(window_size: u32) -> PyResult<Self> {
        Ok(Self {
            inner: Lzxd::new(parse_window_size(window_size)?),
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
    }

    fn decompress_next<'py>(
        &mut self,
        py: Python<'py>,
        chunk: &[u8],
        output_len: usize,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let decoded = self
            .inner
            .decompress_next(chunk, output_len)
            .map_err(|e| PyRuntimeError::new_err(format!("LZX decode failed: {e}")))?;
        Ok(PyBytes::new_bound(py, decoded))
    }
}

#[pyfunction]
#[pyo3(signature = (chunks, output_lengths, window_size=65536u32))]
fn decompress_lzxd_chunks<'py>(
    py: Python<'py>,
    chunks: Vec<Vec<u8>>,
    output_lengths: Vec<usize>,
    window_size: u32,
) -> PyResult<Bound<'py, PyBytes>> {
    if chunks.len() != output_lengths.len() {
        return Err(PyValueError::new_err(
            "chunks and output_lengths must have the same length",
        ));
    }

    let mut lzxd = Lzxd::new(parse_window_size(window_size)?);
    let mut out = Vec::new();

    for (chunk, output_len) in chunks.iter().zip(output_lengths.iter().copied()) {
        let decoded = lzxd
            .decompress_next(chunk, output_len)
            .map_err(|e| PyRuntimeError::new_err(format!("LZX decode failed: {e}")))?;
        out.extend_from_slice(decoded);
    }

    Ok(PyBytes::new_bound(py, &out))
}

fn parse_window_size(window_size: u32) -> PyResult<WindowSize> {
    match window_size {
        32_768 => Ok(WindowSize::KB32),
        65_536 => Ok(WindowSize::KB64),
        131_072 => Ok(WindowSize::KB128),
        262_144 => Ok(WindowSize::KB256),
        524_288 => Ok(WindowSize::KB512),
        1_048_576 => Ok(WindowSize::MB1),
        2_097_152 => Ok(WindowSize::MB2),
        4_194_304 => Ok(WindowSize::MB4),
        8_388_608 => Ok(WindowSize::MB8),
        16_777_216 => Ok(WindowSize::MB16),
        33_554_432 => Ok(WindowSize::MB32),
        _ => Err(PyValueError::new_err(
            "unsupported window_size; use one of: 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432",
        )),
    }
}

#[pymodule]
fn pylzx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LzxdDecoder>()?;
    m.add_function(wrap_pyfunction!(decompress_lzxd_chunks, m)?)?;
    Ok(())
}
