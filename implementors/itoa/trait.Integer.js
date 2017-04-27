(function() {var implementors = {};
implementors["itoa"] = [];
implementors["jsonrpc_core"] = [];
implementors["jsonrpc_http_server"] = [];
implementors["jsonrpc_macros"] = [];
implementors["jsonrpc_pubsub"] = [];
implementors["rpc"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
