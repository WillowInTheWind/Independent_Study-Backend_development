import { Component } from '@angular/core';
import {MatButton, MatFabButton} from "@angular/material/button";
import {MatDialogActions, MatDialogRef} from "@angular/material/dialog";
import {MatIcon} from "@angular/material/icon";
import {NgIf} from "@angular/common";
import {CookieService} from "ngx-cookie-service";

@Component({
  selector: 'app-logout-dialog',
  standalone: true,
    imports: [
        MatButton,
        MatDialogActions,
        MatFabButton,
        MatIcon,
        NgIf
    ],
  templateUrl: './logout-dialog.component.html',
  styleUrl: './logout-dialog.component.css'
})
export class LogoutDialogComponent {
cancel() {
  this.dialogRef.close();

}

logout() {
  this.cookie.delete("token")
  this.dialogRef.close();
  window.location.reload()

}
constructor(    public dialogRef: MatDialogRef<LogoutDialogComponent>,
                private  cookie: CookieService) {
}
}
