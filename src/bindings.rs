use pyo3::prelude::*;
use crate::accelerator::Accelerator;
use crate::request::Request;
use crate::router::Router;
use crate::strategies::{least_connections::LeastConnections, p2c::PowerOfTwoChoices, round_robin::RoundRobin};

#[pyclass(name = "Accelerator")]
#[derive(Clone)]
pub struct PyAccelerator {
    pub inner: Accelerator,
}

#[pymethods]
impl PyAccelerator {
    #[new]
    pub fn new(id: usize, capacity: u32) -> Self {
        PyAccelerator {
            inner: Accelerator::new(id, capacity),
        }
    }

    #[getter]
    pub fn id(&self) -> usize {
        self.inner.id
    }

    #[getter]
    pub fn capacity(&self) -> u32 {
        self.inner.capacity
    }

    #[getter]
    pub fn current_load(&self) -> u32 {
        self.inner.current_load
    }
}

#[pyclass(name = "Request")]
#[derive(Clone)]
pub struct PyRequest {
    pub inner: Request,
}

#[pymethods]
impl PyRequest {
    #[new]
    pub fn new(id: usize, cost: u32, priority: u8) -> Self {
        PyRequest {
            inner: Request::new(id, cost, priority),
        }
    }

    #[getter]
    pub fn id(&self) -> usize {
        self.inner.id
    }

    #[getter]
    pub fn cost(&self) -> u32 {
        self.inner.cost
    }
}

#[pyclass(name = "Router")]
pub struct PyRouter {
    inner: Router,
}

#[pymethods]
impl PyRouter {
    #[new]
    pub fn new(strategy_name: &str) -> PyResult<Self> {
        let strategy: Box<dyn crate::router::LoadBalancingStrategy> = match strategy_name {
            "round_robin" => Box::new(RoundRobin::new()),
            "least_connections" => Box::new(LeastConnections::new()),
            "p2c" => Box::new(PowerOfTwoChoices::new()),
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Unknown strategy")),
        };

        Ok(PyRouter {
            inner: Router::new(strategy),
        })
    }

    pub fn add_accelerator(&mut self, accelerator: &PyAccelerator) {
        self.inner.add_accelerator(accelerator.inner.clone());
    }

    pub fn route_request(&mut self, request: &PyRequest) -> Option<usize> {
        self.inner.route_request(&request.inner)
    }
}

#[pymodule]
fn intelligent_routing(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAccelerator>()?;
    m.add_class::<PyRequest>()?;
    m.add_class::<PyRouter>()?;
    Ok(())
}
