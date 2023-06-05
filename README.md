# OTC - Oh My Posh Theme Chooser For Windows! 

### Installation 
```
cargo install otc
```

<details>
<summary> 
more </summary> 
  
#### Potential Dependencies:
- [Cargo & Rust:](https://doc.rust-lang.org/cargo/getting-started/installation.html)
   
- [Git for Windows](https://gitforwindows.org/)
  

#### Alternative Method
```
git clone https://github.com/nrdrch/otc.git
```
```
cd winfu
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
| --add--newtheme, -a | Add a theme to the list | otc -a "NewName" "/Path/to/theme.omp.json" |
| -help, -h          | Display help   | otc -h |  
---------
