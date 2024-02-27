#pragma once
#include <string>
#include <libssh/libssh.h>
#include <iostream>

class connecttion {
    public:
        /**
            @brief connection server
            @param password
            @param username
            @param ip
            @param port
        */
        connecttion (std::string username, std::string password,
                     std::string ip, std::string port = "22");
                     
        ~connecttion() {
            std::cout << "free ssh" << std::endl;
            ssh_disconnect (this->session);
            ssh_free (this->session);
        }
        /**
            @brief async upload file from source to destination
            @param source
            @param destination
            @return
        */
        bool upload_file (std::string source, std::string destination);
        
        void set_option (ssh_options_e option, std::string value);
        
        bool login (std::string username, std::string password);
        
        bool down_file (std::string source, std::string destination);
        
        std::string run_cmd (std::string cmd);
    private:
        ssh_session session;
};
