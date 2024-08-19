import { ComponentFixture, TestBed } from '@angular/core/testing';

import { EditingmodalComponent } from './editingmodal.component';

describe('EditingmodalComponent', () => {
  let component: EditingmodalComponent;
  let fixture: ComponentFixture<EditingmodalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [EditingmodalComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(EditingmodalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
