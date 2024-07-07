# karasync.nvim
> Under development, not available for public use at the moment.

- [ ] Asynchronous seamless synchronization project
- [ ] Allow simultaneous synchronization to multiple servers
- [ ] Diff comparison (notify and handle differences when server files are modified by others before synchronization)
- [ ] task run project
- [ ] test project
- [ ] start others task in backstage


```doc
   ---<--------------|  
   |                 |    
neovim plugin -> [karasync(lua)] -> Message registration -> karasync(rust) server 
                       ^                                        |
                       |                                        |
                        ------------------------------------- <-
```
