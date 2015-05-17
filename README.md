# rust-trousers

Rust bindings for TrouSerS, an open-source TSS implementation.

## Introduction

TrouSerS is compliant with the [TCG Software Stack (TSS) 1.2](https://www.trustedcomputinggroup.org/resources/tcg_software_stack_tss_specification)
(with minor differences). This library attempts to wrap the lower level
functionality in a more object-oriented interface, as well as reducing the
burden of manual memory management, whilst retaining strong similarities to the
TSS interface. (For example, implementing a singular function to extend a PCR is
out of scope.)

The main `trousers` crate contains the wrapping interface, and depends on the
`trousers-sys` crate, which includes direct bindings by rust-bindgen. The
bindings were generated off TrouSerS 0.3.11.2-1, from the Ubuntu trusty/main
repositories.

Please note that this library has been developed primarily as a side project.
While I would like to achieve full coverage of TrouSerS, this effectively means
full coverage of the TSS, a monumental task to perform alone out of spare time.
The API is also not set in stone at this time, and may change drastically.
Contributions are welcome!

## Coverage

Module | Implemented | Partial | Missing
-------|-------------|---------|--------
4.3.3.1 Common methods | N/A | Tspi_SetAttribData (for TSS_OBJECT_TYPE_RSAKEY) | Tspi_SetAttribUint32, Tspi_GetAttribUint32, Tspi_GetAttribData, Tspi_ChangeAuth, Tspi_ChangeAuthAsym, Tspi_GetPolicyObject
4.3.3.2 Common context methods | Tspi_Context_Create, Tspi_Context_Close, Tspi_Context_FreeMemory, Tspi_Context_GetTPMObject | Tspi_Context_Connect, Tspi_Context_CreateObject (TSS_OBJECT_TYPE_PCRS, ref 2.3.2.1) | Tspi_Context_GetDefaultPolicy, Tspi_Context_CloseObject, Tspi_Context_GetCapability
4.3.4.1 | N/A | N/A | All
4.3.4.2 Finding, Loading, and Registering Keys in a Context | Tspi_Context_LoadKeyByUUID | N/A | Tspi_Context_LoadKeyByBlob, Tspi_Context_RegisterKey, Tspi_Context_UnregisterKey, Tspi_Context_GetKeyByUUID, Tspi_Context_GetKeyByPublicInfo, Tspi_Context_GetRegisteredKeysByUUID, Tspi_Context_GetRegisteredKeysByUUID2, Tspi_TPM_KeyControlOwner
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
4.3.4.13 Old PCR commands [(trivia)][1] | Tspi_TPM_PcrRead | Tspi_TPM_PcrExtend | Tspi_TPM_GetEvent, Tspi_TPM_GetEvents, Tspi_TPM_GetEventLog, Tspi_TPM_Quote
4.3.4.14 | N/A | N/A | All
4.3.4.15 Tspi_PcrComposite Class | Tspi_PcrComposite_SelectPcrIndex | N/A | Tspi_SetAttribUint32, Tspi_GetAttribUint32, Tspi_PcrComposite_SetPcrValue, Tspi_PcrComposite_GetPcrValue
4.3.4.16 New PCR commands | Tspi_TPM_PcrReset, Tspi_PcrComposite_SelectPcrIndexEx | N/A | Tspi_Data_Seal, Tspi_Data_SealX, Tspi_TPM_Quote2, Tspi_PcrComposite_SetPcrLocality, Tspi_PcrComposite_GetPcrLocality, Tspi_PcrComposite_GetCompositeHash
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

[1]: http://sourceforge.net/p/trousers/mailman/message/18846127/
