//! Trait Interface Definitions for XPS JSON-RPC

use jsonrpsee::{proc_macros::rpc, types::ErrorObjectOwned};

use crate::types::{GrantInstallationResult, Message, Signature};
use ethers::prelude::*;

/// XPS JSON-RPC Interface Methods
#[rpc(server, client, namespace = "xps")]
pub trait Xps {
    // Placeholder for send_message, see [the discussion](https://github.com/xmtp/xps-gateway/discussions/11)
    #[method(name = "sendMessage")]
    async fn send_message(&self, _message: Message) -> Result<(), ErrorObjectOwned>;

    /// # Documentation for JSON RPC Endpoint: `grantInstallation`

    /// ## Overview

    /// The `grantInstallation` method is used to register an installation on the network and associate the installation with a concrete identity.

    /// ## JSON RPC Endpoint Specification

    /// ### Method Name
    /// `grantInstallation`

    /// ### Request Parameters
    /// did: string
    /// name: String,
    /// value: String,
    /// signature: Signature,

    /// ### Request Format
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "status",
    ///   "id": 1
    /// }
    /// ```

    /// - `jsonrpc`: Specifies the version of the JSON RPC protocol being used. Always "2.0".
    /// - `method`: The name of the method being called. Here it is "grantInstallation".
    /// - `id`: A unique identifier established by the client that must be number or string. Used for correlating the response with the request.

    /// ### Response Format
    /// The response will typically include the result of the operation or an error if the operation was unsuccessful.

    /// #### Success Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": "OK",
    ///   "id": 1
    /// }
    /// ```

    /// - `result`: Contains data related to the success of the operation. The nature of this data can vary based on the implementation.

    /// #### Error Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "error": {
    ///     "code": <error_code>,
    ///     "message": "<error_message>"
    ///   },
    ///   "id": 1
    /// }
    /// ```

    /// - `error`: An object containing details about the error.
    ///   - `code`: A numeric error code.
    ///   - `message`: A human-readable string describing the error.

    /// ### Example Usage

    /// #### Request
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "status",
    ///   "id": 42
    /// }
    /// ```

    /// #### Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": "OK",
    ///   "id": 42
    /// }
    /// ```

    /// ### Command Line Example
    /// ```bash
    /// $ $ curl -H "Content-Type: application/json" -d '{"id":7000, "jsonrpc":"2.0", "method":"xps_status"}' http:///localhost:34695
    /// {"jsonrpc":"2.0","result":"OK","id":7000}
    /// ```

    /// ### Notes
    /// - The system should have proper error handling to deal with invalid requests, unauthorized access, and other potential issues.
    #[method(name = "grantInstallation")]
    async fn grant_installation(
        &self,
        did: String,
        name: String,
        value: String,
        signature: Signature,
    ) -> Result<GrantInstallationResult, ErrorObjectOwned>;

    /// # Documentation for JSON RPC Endpoint: `status`

    /// ## Overview

    /// The `status` method is used to query the gateway status.

    /// ## JSON RPC Endpoint Specification

    /// ### Method Name
    /// `status`

    /// ### Request Parameters
    /// none

    /// ### Request Format
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "status",
    ///   "id": 1
    /// }
    /// ```

    /// - `jsonrpc`: Specifies the version of the JSON RPC protocol being used. Always "2.0".
    /// - `method`: The name of the method being called. Here it is "status".
    /// - `id`: A unique identifier established by the client that must be number or string. Used for correlating the response with the request.

    /// ### Response Format
    /// The response will typically include the result of the operation or an error if the operation was unsuccessful.

    /// #### Success Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": "OK",
    ///   "id": 1
    /// }
    /// ```

    /// - `result`: Contains data related to the success of the operation. The nature of this data can vary based on the implementation.

    /// #### Error Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "error": {
    ///     "code": <error_code>,
    ///     "message": "<error_message>"
    ///   },
    ///   "id": 1
    /// }
    /// ```

    /// - `error`: An object containing details about the error.
    ///   - `code`: A numeric error code.
    ///   - `message`: A human-readable string describing the error.

    /// ### Example Usage

    /// #### Request
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "method": "status",
    ///   "id": 42
    /// }
    /// ```

    /// #### Response
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "result": "OK",
    ///   "id": 42
    /// }
    /// ```

    /// ### Command Line Example
    /// ```bash
    /// $ $ curl -H "Content-Type: application/json" -d '{"id":7000, "jsonrpc":"2.0", "method":"xps_status"}' http:///localhost:34695
    /// {"jsonrpc":"2.0","result":"OK","id":7000}
    /// ```

    /// ### Notes
    /// - The system should have proper error handling to deal with invalid requests, unauthorized access, and other potential issues.
    #[method(name = "status")]
    async fn status(&self) -> Result<String, ErrorObjectOwned>;

    #[method(name = "walletAddress")]
    async fn wallet_address(&self) -> Result<Address, ErrorObjectOwned>;
}
