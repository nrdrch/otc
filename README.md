# OTC - Oh My Posh Theme Chooser! 
> (currently only Windows!)
### Installation 
```
cargo install otc
```

<details>
<summary> 
alternative Installation </summary> 
  
#### Potential Dependencies:
- [Cargo & Rust:](https://doc.rust-lang.org/cargo/getting-started/installation.html)
   
- [Git for Windows](https://gitforwindows.org/)
  

#### Other Method
```
git clone https://github.com/nrdrch/otc.git
```
```
cd otc
```
```
cargo build --release
```
- Preferably move the executable from target/release into a directory in your 'Path' enviorment variable for easy execution.

</details>

------------------
| **Option**       | **Description**    | **Example**       |
| :---      | :---          | :---            |
| --list-themes, -ls | List Themes   | otc -ls |     
| --update-omp, -u    | Update OMP including themes | otc -u    |
| --choose-theme, -ch | Choose theme by name | otc -ch bubblesextra   |
| --add-newtheme, -a | Add a theme to the list | otc -a "Name" "/Path/to/theme" |
| -help, -h          | Display help   | otc -h |  
---------


<details>
<summary> 
further explanation </summary> 

### List 
- This lists all available themes in the default OMP Path. 
- Additionally the themes in WindowsPowerShell\ompthemes, that get created by using 
**--add-newtheme**.
> For easier usage and overview, the themes get trimmed of thier file extension before listing.  

command alias: -ls
```
otc --list-themes 
```
### Update
- This will update OMP using winget.
- As well as sync the themes again with thier Github, making sure all changes get applied.

command alias: -u
```
otc --update-omp
```

### Choose
- In this example, we select the theme "amro" to be our new theme.
- This will overwrite the file named "custom.*" with the amro theme, thus selecting it as current theme.
    - Other than isolating added and default themes, this means in the PowerShellProfile, only the file named "custom" has to be imported as theme.
>  Don't include file extensions when choosing as well. This will also choose from the newly created themes. 

command alias: -ch
```
otc --choose-theme amro
```

### Add New
- The first example creates a new theme called "amroedit" from the 
current theme.
- The second example, creates a theme called "test" and includes a path to the theme.
> As mentioned before, the themes get saved to WindowsPowerShell\ompthemes

command alias: -a
```
otc --add-newtheme amroedit
```
or
```
otc --add-newtheme test "path\to\themefile.extension"
```

</details>