# rust-trousers

Rust bindings for TrouSerS, an open-source TSS implementation.
A work in progress.

## Introduction

TrouSerS is compliant with the TCG Software Stack (TSS) 1.2 (with minor
differences), which can be found [here](https://www.trustedcomputinggroup.org/resources/tcg_software_stack_tss_specification).
This library attempts to wrap the lower level functionality in a more idiomatic,
abstracted way. The ideal: freeing the developer from handles, pointers,
pointers-to-pointers, and other memory management and boilerplate code.

## Coverage

At the moment this library is *very* incomplete. The TSS spec is exhaustive
(742 pages), so there is a lot to go through. The below table indicates what
functionality is available.

Module | Implemented | Partial | Missing
-------|-------------|---------|--------
4.3.3.1 Common methods | N/A | N/A | All
4.3.3.2 Common context methods | Tspi_Context_Create, Tspi_Context_Close, Tspi_Context_FreeMemory, Tspi_Context_GetTPMObject | Tspi_Context_Connect | Tspi_SetAttribUint32, Tspi_GetAttribUint32, Tspi_SetAttribData, Tspi_GetAttribData, Tspi_Context_GetDefaultPolicy, Tspi_Context_CreateObject, Tspi_Context_CloseObject, Tspi_Context_GetCapability
4.3.4.3 | N/A | N/A | All
4.3.4.4 | N/A | N/A | All
4.3.4.5 | N/A | N/A | All
4.3.4.6 | N/A | N/A | All
4.3.4.7 | N/A | N/A | All
4.3.4.8 | N/A | N/A | All
4.3.4.9 | N/A | N/A | All
4.3.4.10 | N/A | N/A | All
4.3.4.11 | N/A | N/A | All
4.3.4.12 | N/A | N/A | All
4.3.4.13 | Tspi_TPM_PcrRead | N/A | Tspi_TPM_GetEvent, Tspi_TPM_GetEvents, Tspi_TPM_GetEventLog, Tspi_TPM_Quote, Tspi_TPM_PcrExtend, Tspi_TPM_PcrRead
4.3.4.14 | N/A | N/A | All
4.3.4.15 | N/A | N/A | All
4.3.4.16 | N/A | N/A | All
4.3.4.17 | N/A | N/A | All
4.3.4.18 | N/A | N/A | All
4.3.4.19 | N/A | N/A | All
4.3.4.20 | N/A | N/A | All
4.3.4.21 | N/A | N/A | All
4.3.4.22 | N/A | N/A | All
4.3.4.23 | N/A | N/A | All
4.3.4.24 | N/A | N/A | All
4.3.4.25 | N/A | N/A | All
4.3.4.26 | N/A | N/A | All
4.3.4.27 | N/A | N/A | All
4.3.4.28 | N/A | N/A | All
4.3.4.29 | N/A | N/A | All
4.3.4.30 | N/A | N/A | All
4.3.4.31 | N/A | N/A | All
4.3.4.32 | N/A | N/A | All
4.3.4.33 | N/A | N/A | All
4.3.4.34 | N/A | N/A | All
*Others* | N/A | N/A | All
