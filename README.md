
# duck123acb Chess

A work in progress chess bot with a GUI  

You can play a game as white against a bot as black, but the bot SUCKS at openings (and in general, all it knows how to do is count material and play short checkmates)  
If you want a different starting position, you have to change the `FEN` constant in the `main.rs` file

### WIP notes:
bot plays more natural moves, but is still pretty bad in the opening  
also for some reason my check detection is still buggy so it might crash when you play against it D:

![image](https://github.com/user-attachments/assets/3f5d5d17-c98d-493b-a914-a757899dc7ae)
![image](https://github.com/user-attachments/assets/d6b2831f-8083-4ae9-901d-3a637cf0bb14)
![image](https://github.com/user-attachments/assets/768ce622-47f6-4949-a545-7e9c397035b5)
![image](https://github.com/user-attachments/assets/f0773b2c-434b-4a6a-b5a4-32d28819cd44)

## Run Locally

Clone the project

```bash
  git clone https://github.com/duck123acb/chess
```

Go to the project directory

```bash
  cd chess
```

Install dependencies & Run

```bash
  cargo run --release
```

