import {Component, Inject} from '@angular/core';
import {AuthorizationService} from "../authorization.service";
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {MAT_DIALOG_DATA, MatDialog, MatDialogRef} from "@angular/material/dialog";
import {Observable} from "rxjs";
import {MxinfoComponent} from "../mxinfo/mxinfo.component";

@Component({
  selector: 'app-deletepage',
  standalone: true,
  imports: [],
  templateUrl: './deletepage.component.html',
  styleUrl: './deletepage.component.css'
})
export class DeletepageComponent {
  delete( ) {
    this.morningex.deleteMx(this.id);
    this.dialogref.close();
    this.modal.close()
    // window.location.reload();
  }
  title: string = "";
  id: number = 0;
  constructor( private user: AuthorizationService, public modal: MatDialogRef<MxinfoComponent>,public dialogref: MatDialogRef<DeletepageComponent>, public morningex: MorningExService, @Inject(MAT_DIALOG_DATA) protected document: Observable<MorningExercise>,) {
    document.subscribe(mx => {
      this.title = mx.title;
      this.id = mx.id;
    });
  }
}
