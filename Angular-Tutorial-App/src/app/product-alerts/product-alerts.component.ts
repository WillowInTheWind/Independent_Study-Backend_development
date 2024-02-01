import { Product } from '../products';
import { Component, Input, Output, EventEmitter } from '@angular/core';

@Component({
  selector: 'app-product-alerts',
  standalone: false,
  // imports: [],
  templateUrl: './product-alerts.component.html',
  styleUrl: './product-alerts.component.css',
})
export class ProductAlertsComponent {
  @Input() product: Product | undefined;
  @Output() notify = new EventEmitter();
}
