(function() {var implementors = {};
implementors["db"] = [];
implementors["jsonrpc_macros"] = [];
implementors["jsonrpc_pubsub"] = [];
implementors["owning_ref"] = [];
implementors["p2p"] = [];
implementors["parking_lot"] = [];
implementors["rpc"] = [];
implementors["verification"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
