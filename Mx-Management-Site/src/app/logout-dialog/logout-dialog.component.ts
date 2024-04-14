import {Component, Inject} from '@angular/core';
import {MatButton, MatFabButton} from "@angular/material/button";
import {MatDialogActions, MatDialogRef} from "@angular/material/dialog";
import {MatIcon} from "@angular/material/icon";
import {DOCUMENT, NgIf} from "@angular/common";
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
  this.cookie.delete("token", '/')
  this.dialogRef.close();
  this.document.location.href = '/'


}
constructor(    public dialogRef: MatDialogRef<LogoutDialogComponent>,
                private  cookie: CookieService, @Inject(DOCUMENT) private document: Document,) {
}
}
