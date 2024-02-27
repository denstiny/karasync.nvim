#include "connection.hpp"
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <iostream>
#include <libssh/libssh.h>
#include <libssh/sftp.h>
#include <sys/stat.h>
#include <vector>

connecttion::connecttion (std::string username, std::string password,
                          std::string ip, std::string port) {
    this->session = ssh_new();
    set_option (SSH_OPTIONS_HOST, ip.c_str());
    set_option (SSH_OPTIONS_PORT_STR, port.c_str());
    
    std::cout << "connect server" << std::endl;
    int result = ssh_connect (this->session);
    if (result == SSH_OK) {
        std::cout << "connect server successful" << std::endl;
    } else {
        std::cerr << "connect server lose" << std::endl;
        exit (-1);
    }
    
    login (username, password);
}

void connecttion::set_option (ssh_options_e option, std::string value) {
    ssh_options_set (this->session, option, value.c_str());
}

bool connecttion::down_file (std::string source, std::string destination) {
    ssh_scp scp_session = ssh_scp_new (this->session, SSH_SCP_READ,
                                       destination.c_str());
    if (scp_session == NULL) {
        std::cout << "Open scp session lose" << std::endl;
        return false;
    } else {
        std::cout << "Open scp session Success" << std::endl;
    }
    
    int result = ssh_scp_init (scp_session);
    if (result == SSH_OK) {
        std::cout << "ssh scp init successful" << std::endl;
    } else {
        std::cerr << "ssh scp init lose" << std::endl;
        return false;
    }
    
    result =  ssh_scp_pull_request (scp_session);
    if (result != SSH_SCP_REQUEST_NEWFILE) {
        return false;
    }
    
    int server_file_size = ssh_scp_request_get_size (scp_session);
    std::string server_file_name = strdup (ssh_scp_request_get_filename (
            scp_session));
    int filePermission  = ssh_scp_request_get_permissions (scp_session);
    
    std::vector<char> buf;
    ssh_scp_accept_request (scp_session);
    result = ssh_scp_read (scp_session, buf.data(), server_file_size);
    if (result == SSH_ERROR) {
        std::cerr << "read server file lose" << std::endl;
        return false;
    } else {
        std::cout << "read server file successful" << std::endl;
    }
    std::fstream fin (source, std::ios::in | std::ios::trunc);
    fin.write (buf.data(), buf.size());
    
    ssh_scp_close (scp_session);
    ssh_scp_free (scp_session);
    return true;
}

bool connecttion::upload_file (std::string source, std::string destination) {
    std::ifstream fin (source, std::ios::binary);
    if (fin) {
        fin.seekg (0, std::ios::end);
        std::ios::pos_type bufsize = fin.tellg();
        fin.seekg (0);
        ssh_scp scp_session = ssh_scp_new (this->session, SSH_SCP_WRITE | SSH_SCP_READ,
                                           ".");
        if (scp_session == NULL) {
            std::cerr << "Open scp session lose" << std::endl;
            return false;
        } else {
            std::cout << "Open scp session Success" << std::endl;
        }
        
        // 打开远程服务器文件
        int result = ssh_scp_push_file (scp_session, destination.c_str(), bufsize,
                                        S_IRUSR | S_IWUSR);
        if (result == SSH_OK) {
            std::cout << "create server file successful" << std::endl;
        } else {
            std::cerr << "create server file lose" << std::endl;
            return false;
        }
        
        // 将文件内容写入服务端文件
        std::vector<char> buf;
        fin.read (buf.data(), bufsize);
        result = ssh_scp_write (scp_session, buf.data(), bufsize);
        if (result == SSH_OK) {
            std::cout << "file copy successful" << std::endl;
        } else {
            std::cout << "file copy lose" << std::endl;
            return false;
        }
        ssh_scp_close (scp_session);
        ssh_scp_free (scp_session);
    }
    return true;
}

bool connecttion::login (std::string username, std::string password) {
    int result = ssh_userauth_password (this->session, username.c_str(),
                                        password.c_str());
    if (result == SSH_AUTH_SUCCESS) {
        std::cout << "login server successful" << std::endl;
    } else {
        std::cerr << "login server lose" << std::endl;
        return false;
    }
    return true;
}

std::string connecttion::run_cmd (std::string cmd) {
    std::string result = "";
    ssh_channel channel = ssh_channel_new (this->session);
    ssh_channel_open_session (channel);
    ssh_channel_request_exec (channel, cmd.c_str());
    char buffer[1024];
    int n = ssh_channel_read (channel, buffer, sizeof (buffer), 0);
    while (n > 0) {
        result.append (buffer);
        n = ssh_channel_read (channel, buffer, sizeof (buffer), 0);
    }
    ssh_channel_close (channel);
    return result;
}
