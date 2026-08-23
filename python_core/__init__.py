"""
DIRECTIVE-X: Python Orchestrator Initialization
This module provides the Python-bindings to the native Rust-bus.
"""

from .orchestrator import HiveMindOrchestrator, AgentTask

__version__ = "1.0.0"
__all__ = ["HiveMindOrchestrator", "AgentTask"]

print("[PYTHON CORE] Loaded DIRECTIVE-X Hive Mind bindings.")
