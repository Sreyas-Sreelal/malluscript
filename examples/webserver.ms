http_plugin http ennu ulppeduthuka;

connected(id) {
    "New connection received with client ID: " ezhuthuka;
    id ezhuthuka;
    "\n" ezhuthuka;
}

"Starting web server on port 3000...\n" ezhuthuka;
server_id = http.http_listen<3000>;

1 um 1 um thullyamanengil avarthikuga {
    "Waiting for connections...\n" ezhuthuka;
    client_id = http.http_accept<server_id>;
    
    0 ne_kal client_id veluthan enkil {
        request = http.http_read<client_id>;
        
        "Received request from client.\n" ezhuthuka;
        
        body = "<!DOCTYPE html><html><head><style>body { font-family: system-ui, sans-serif; background: #0f172a; color: #f8fafc; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; } .card { background: rgba(255,255,255,0.05); padding: 3rem; border-radius: 1rem; box-shadow: 0 4px 30px rgba(0,0,0,0.1); border: 1px solid rgba(255,255,255,0.1); text-align: center; } h1 { background: linear-gradient(to right, #38bdf8, #818cf8); -webkit-background-clip: text; color: transparent; margin-top: 0; }</style></head><body><div class=\"card\"><h1>Malluscript Server!</h1><p>Served natively via the C-ABI HTTP Plugin.</p></div></body></html>";
        
        response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nConnection: close\r\n\r\n";
        http.http_write<client_id, response>;
        http.http_write<client_id, body>;
        http.http_close<client_id>;
        
        "Response sent and connection closed.\n" ezhuthuka;
    }
}

