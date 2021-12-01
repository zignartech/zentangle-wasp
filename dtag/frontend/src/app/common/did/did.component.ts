import { Component } from '@angular/core';

@Component({
  selector: 'app-did',
  templateUrl: './did.component.html',
  styleUrls: ['./did.component.scss'],
})
export class DIDComponent {
  showDIDDetails = false;

  onClickUserDID(): void {
    this.showDIDDetails = true;
  }
}
